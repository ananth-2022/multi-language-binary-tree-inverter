class Node:
    def __init__(self, value, left, right):
        self.value = value
        self.left = left
        self.right = right
    
    def invert(self):
        self.left, self.right = self.right, self.left
        if self.left:
            self.left.invert()
        if self.right:
            self.right.invert()
    
    def __str__(self):
        return f'Node(value={self.value}, l={self.left}, r={self.right})'

tree = Node(3, Node(5, Node(7, None, None), Node(1, None, None)), Node(8, None, None))
print(tree)
tree.invert()
print(tree)