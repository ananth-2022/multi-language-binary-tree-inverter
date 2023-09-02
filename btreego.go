package main

import (
	"fmt"
)

type Node struct {
	value int
	left *Node
	right *Node
}

func (n *Node)invert() {
	n.left, n.right = n.right, n.left
	if n.left != nil {
		n.left.invert()
	}
	if n.right != nil {
		n.right.invert()
	}
}

func main() {
	tree := &Node{1, &Node{2, &Node{4,nil,nil}, &Node{5,nil,nil}}, &Node{3, &Node{6,nil,nil}, &Node{7,nil,nil}}}
	fmt.Println(*tree,*tree.left,*tree.right)
	tree.invert()
	fmt.Println(*tree,*tree.left,*tree.right)
}