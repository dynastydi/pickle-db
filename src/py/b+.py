class Tree:
    def __init__(self, vals : dict = None):
        if vals:
            print(vals)

        self.children = []

t = Tree("??")


class Node:

    def __init__(self, val):
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
        

