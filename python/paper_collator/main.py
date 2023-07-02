import os
import sys
import paper

def input_dir(): 
    prompt = "Please input a valid dir containing papers: ";
    dir = input(prompt)
    while not os.path.isdir(dir):
        print(f"The dir[{dir}] is not valid.")
        dir = input(prompt)
    ab_path = os.path.abspath(dir)
    print("The dir is: " + ab_path)
    return ab_path

def collect_papers(ab_dir):
    papers = []
    for root, dirnames, files in os.walk(ab_dir):
        for file in files:
            if file.endswith(".pdf"):
                path = os.path.join(root, file)
                size = os.path.getsize(path)
                p = paper.Paper(path,file, size=size)
                papers.append(p)
    print(f"=======\nFound {len(papers)} papers.\n")
    return papers

def find_similar_papers(papers):
    found = False
    for i in range(0, len(papers)):
        for j in range(i+1, len(papers)):
            if(papers[i].is_the_same_file(papers[j])):
                print("==="*10,"\n", "Found the same papers:")
                print("\t", f'open "{papers[i].path}"', " <--> ", f'open "{papers[j].path}"')
                found = True
                continue
            if(papers[i].is_very_like(papers[j])):
                print("==="*10,"\n", "Found the similar papers:")
                print("\t", f'open "{papers[i].path}"', " <--> ", f'open "{papers[j].path}"')
                found = True
    if found == False:
        print("No similar papers found")

def run():
    ab_dir = input_dir()
    papers = collect_papers(ab_dir)
    find_similar_papers(papers)
    print("\n--------------All done---------------")

def program_exit():
    print("\n"+"="*10 + "Paper collator ends" + "="*10)
    sys.exit(0)

if __name__ == "__main__":
    print("="*10 + "Paper collator starts" + "="*10)
    try:
        run()
    except KeyboardInterrupt:
        program_exit()
    except EOFError:
        program_exit()