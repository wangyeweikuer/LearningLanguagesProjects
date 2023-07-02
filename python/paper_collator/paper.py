import re
import unittest
import hashlib
import sys

def normalize_remarked(file) -> tuple:
    m = re.match('^(.*)(_|-)?remarked(_|-)?(.*)$',file)
    if m == None:
        return file, False
    else:
        return m.group(1)+m.group(4), True

def normalize_year(file) -> tuple:
    m = re.match('^(.*)(_|-)?\[(\d{4})\](_|-)?(.*)$',file)
    if m == None:
        return file, None
    else:
        return m.group(1)+m.group(5), int(m.group(3))

def normalize_citation(file) -> tuple:
    m = re.match('^(.*)(_|-)?\[(<\d+|\d+\+)\](_|-)?(.*)$',file)
    if m == None:
        return file, None
    else:
        c = m.group(3)
        n = None
        if c.startswith('<'):
            n = int(c[1:])
        else:
            n = int(c[:-1])
        return m.group(1)+m.group(5), n

def normalize_vendor(file) -> tuple:
    vendors = set(['alibaba','google','facebook','microsoft','amazon', 
                   'aws', 'oracle', 'apple','tencent','huawei','baidu',
                   'yale','berkeley','harvard','stanford','ucberkeley','cmu','ucla'])
    pattern = '^(.*)(_|-)?\[('
    for v in vendors:
        pattern += v+'|'
    pattern += ')\](_|-)?(.*)$'
    m = re.match(pattern, file)
    if m == None:
        return file, None
    else:
        return m.group(1)+m.group(5), m.group(3)

def normalize_conference(file) -> tuple:
    cs = ['sigmod','vldb','icde','atc','osdi','nsdi','aaai','bpod','eecs']
    pattern = '^(.*)(_|-)?\[?('
    first = True
    for c in cs:
        if first:
            first = False
        else:
            pattern += '|'
        pattern += c+"-?\d{2,4}|"
        pattern += c+"_?\d{2,4}|"
        pattern += "\d{2,4}-?"+c+"|"
        pattern += "\d{2,4}_?"+c
    pattern += ')\]?(_|-)?(.*)$'
    # print(pattern)
    m = re.match(pattern, file)
    if m == None:
        return file, None, None
    else:
        new_file = m.group(1)+m.group(5);
        conf = m.group(3)
        # print(conf)
        year = None
        def calc_year(str):
            if len(str) ==0:
                return None
            # print(str)
            str = re.sub('[-_]', '', str)
            y = int(str)
            if(y>1000):
                return y
            elif(y < 70):
                y += 2000
            else:
                y += 1900
            return y
        for c in cs:
            if conf.startswith(c):
                year = calc_year(conf[len(c):])
                conf = c
                break
            if conf.endswith(c):
                year = calc_year(conf[:-len(c)])
                conf = c
                break
    return (new_file, conf, year)

def human_readable_size(size):
    if size == None:
        return None
    if size < 1024:
        return str(size) + 'B'
    elif size < 1024 * 1024:
        return str(size / 1024) + 'KB'
    elif size < 1024 * 1024 * 1024:
        return str(size / (1024 * 1024)) + 'MB'
    else:
        return str(size / (1024 * 1024 * 1024)) + 'GB'

def normalize_file(file):
    file = re.sub('.pdf', '', file)
    file = re.sub('_|-|\[|\]|\*|#', ' ', file)
    file = re.sub(' {2,}', ' ', file)
    file = file.strip()
    return file

def calc_md5(path):
    with open(path, 'rb') as f:
        return hashlib.md5(f.read()).hexdigest()

class Paper:

    def __init__(self, path, file, size=None):
        self.path = path
        self.size = size
        # file = re.sub('\[|\]','-',file)
        self.readable_size = human_readable_size(size)
        (file, year) = normalize_year(file.lower())
        self.year = year
        (file, conference, year) = normalize_conference(file.lower())
        self.conference = conference
        if(year):
            self.year = year
        (file, citation) = normalize_citation(file.lower())
        self.citation = citation
        (file, vendor) = normalize_vendor(file.lower())
        self.vendor = vendor
        (file, remarked) = normalize_remarked(file.lower())
        self.remarked = remarked
        self.name = normalize_file(file)
        self.name_len = len(self.name)
        self.md5 = calc_md5(path)
        assert self.name_len > 0
        
    def __str__(self) -> str:
        return f"""
    Path={self.path}
    Name={self.name} 
    Citation={self.citation} 
    Vendor={self.vendor} 
    Remarked={self.remarked} 
    Conference={self.conference} 
    Year={self.year}
    Size={self.size}
    ReadableSize={self.readable_size}
    """
    
    def calc_edit_distance(self, other):
        len1 = self.name_len
        len2 = other.name_len
        # dist = [[0]*(len2 + 1)]*2
        dist = [[0]*(len2 + 1), [0]*(len2 + 1)]
        dist[0][0]= 0
        for j in range(1, len2 +1):
            dist[0][j] = j
        for i in range(1, len1 +1):
            c1 = self.name[i - 1]
            dist[i%2][0] = i
            for j in range(1, len2 +1):
                c2 = other.name[j -1]
                # print(f'{c1} {c2}')
                # print(f'{dist[(i-1)%2][j-1]} {dist[(i-1)%2][j]} {dist[i%2][j-1]}')
                if c1 == c2:
                    dist[i%2][j] = dist[(i-1)%2][j-1]
                else:
                    dist[i%2][j] = min(dist[(i-1)%2][j-1], dist[(i-1)%2][j], dist[i%2][j-1]) + 1
                # print(f"dist[{i}][{j}] = {dist[i%2][j]}")
        # print(dist)
        return dist[len1%2][len2]
    
    def is_very_like(self, other, likehood = 0.8):
        dist = self.calc_edit_distance(other)
        m = min(self.name_len, other.name_len)
        return (1-dist/m)>=likehood
    
    def is_the_same_file(self, other):
        if self.size != None and other.size != None and self.size == other.size:
            return self.md5 != None and other.md5 != None and self.md5 == other.md5

if __name__ == '__main__':
    files = [
        "Who Limits the Resource Efficiency of My Datacenter- An Analysis of Alibaba Datacenter Traces_[2019]_[alibaba].pdf",
        "The CQL Continuous Query Language- Semantic Foundations and Query Execution_[2003]_[1700+].pdf",
        "RunD- A Lightweight Secure Container Runtime for High-density Deployment and High-concurrency Startup in Serverless Computing-[atc2022]-remarked.pdf",
        "Relational Model of Data Large Shared Data Banks-[1970]-[13600+].pdf",
        "Occupy the Cloud- Distributed Computing for the 99%.pdf",
        "Implementing Data Cubes Efficiently-[1999]-[2200+].pdf",
        "Firecracker- Lightweight Virtualization for Serverless Applications-[nsdi20]-[300+].pdf",
        "Distributed Data-Parallel Computing Using a High-Level Programming Language_sigmod09_[microsoft].pdf",
        "Convolutional Neural Networks over Tree Structures for Programming Language Processing-[2016-AAAI]-DL4program.pdf",
        "Cloud Programming Simplified- A Berkeley View on Serverless Computing--[[berkeley]-[EECS-2019-3].pdf",
        "A bridging model for parallel computation_[1990]_[5100+].pdf",
        "A   bridging model for    parallel computation.pdf"
    ]
    # p1 = Paper(files[0],files[0])
    # print(p1)
    # p2 = Paper(files[1],files[1])
    # print(p2)
    # print(p1.calc_edit_distance(p2))

    papers = []
    for f in files:
        papers.append(Paper(f,f))
    found = False;
    for i in range(0, len(papers)):
        for j in range(i+1, len(papers)):
            if(papers[i].is_very_like(papers[j])):
                print("==="*10,"\n", "Found similar papers:")
                print("\t", papers[i].path, " <--> ", papers[j].path)
                found = True
    if found == False:
        print("No similar papers found")