#include <iostream>

/**
 * Definition for singly-linked list.
 */

struct ListNode
{
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution
{
public:
    ListNode *mergeTwoLists(ListNode *l1, ListNode *l2)
    {
        if(l1 == nullptr){
            return l2;
        }else if(l2 == nullptr){
            return l1;
        }
        ListNode head = ListNode(-2000);
        ListNode* now = &head;
        while (l1 || l2)
        {
            if(l1 == nullptr){
                now->next = l2;
                l2 = l2->next;
            }else if(l2 == nullptr){
                now->next = l1;
                l1=l1->next;
            } else if(l1 ->val > l2->val){
                now->next = l2;
                l2 = l2->next;
            }
            else{
                now->next = l1;
                l1=l1->next;
            }
            now = now->next;
        }
        return head.next;
    }
};

int main()
{
    using std::cout;
    using std::endl;
    ListNode l1 = ListNode(1);
    cout << (&l1) <<endl;
    ListNode h1 = ListNode(3);
    cout << (&h1) <<endl;
    l1.next = &h1;
    ListNode h2 = ListNode(5);
    cout << (&h2) <<endl;
    l1.next->next = &h2;

    ListNode l2 = ListNode(1);
    cout << (&l2) <<endl;
    ListNode h3 = ListNode(2);
    cout << (&h3) <<endl;
    l2.next = &h3;
    ListNode h4 = ListNode(3);
    cout << (&h4) <<endl;
    l2.next->next = &h4;
    ListNode h5 = ListNode(6);
    cout << (&h5) <<endl;
    l2.next->next->next = &h5;

    Solution s;
    ListNode* ptr = s.mergeTwoLists(&l1,&l2);
    using std::cout;
    while(ptr){
        cout << ptr->val << " ";
        ptr = ptr->next;
    }
    cout << std::endl;
    return 1;
}
