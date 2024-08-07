

class Node:

    def __str__(self):
        
        if self.left:
            print(str(self.left))
        
        return str(self.val)


    def __init__(self, val=None):
        self.left = None
        self.right = None
        self.val = val

    def insert(self, val):
        
        if not self.val:
            self.val = val
        

        elif val < self.val:

            if self.left:
                self.left.insert(val)

            else:
                self.left = Node(val)


        elif val > self.val:

            if self.right:
                self.right.insert(val)
            else:
                self.right = Node(val)
        

tree = Node(15)

tree.insert(1)
tree.insert(87)
tree.insert(23)

print(str(tree))
