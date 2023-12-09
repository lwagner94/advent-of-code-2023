package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type Day09Calculator struct {
	layers [][]int
}

func NewDay09Calculator(line string) (*Day09Calculator, error) {
	c := new(Day09Calculator)

	var first []int

	for _, number := range strings.Split(line, " ") {
		num, err := strconv.Atoi(number)
		if err != nil {
			return nil, err
		}

		first = append(first, num)
	}

	c.layers = append(c.layers, first)
	return c, nil
}

func (c *Day09Calculator) Calculate() int {
	layer := 0

	for {
		length := len(c.layers[layer]) - 1
		next := make([]int, length)

		all_zero := true

		for i := 0; i < length; i++ {
			diff := c.layers[layer][i+1] - c.layers[layer][i]

			if diff != 0 {
				all_zero = false
			}

			next[i] = diff
		}

		layer += 1
		c.layers = append(c.layers, next)

		if all_zero {
			break
		}
	}

	c.layers[layer] = append(c.layers[layer], 0)
	layer -= 1

	var last int
	for {
		last = c.layers[layer][len(c.layers[layer])-1] +
			c.layers[layer+1][len(c.layers[layer+1])-1]

		c.layers[layer] = append(c.layers[layer], last)

		if layer == 0 {
			break
		}

		layer -= 1
	}

	return last
}

func main() {
	file, err := os.Open("example")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	sum := 0

	for scanner.Scan() {
		line := scanner.Text()

		c, err := NewDay09Calculator(line)
		if err != nil {
			fmt.Println(err)
		}

		sum += c.Calculate()
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	fmt.Printf("Sum Part 1: %v\n", sum)
}
