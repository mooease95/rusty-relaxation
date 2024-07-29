# Relaxation

This relaxation technique uses the Jacobi method of determining the solutions of linear equations.

## A terrible explanation

We start with a 2D array (array of array) where the border elements are solved, and the middle elements are 0 (we could start with anything, really). We iterate over the middle elements, replacing them with the average of their neighbours, which gives an approximate value.

Continuously approximating them in an iterative approach causes the middle elements to converge to the correct value.

## Example

Given a function f(x,y) = x + y, and 4x4 2D array where x is the row number going horizontally, and y is the column number going vertically, this is what the solved array would look like (row and column headers are in ***bold, italicised.***:

|         | ***0*** | ***1*** | ***2*** | ***3*** |
|---------|---------|---------|---------|---------|
| ***0*** | 0       | 1       | 2       | 3       |        
| ***1*** | 1       | 2       | 3       | 4       |
| ***2*** | 2       | 3       | 4       | 5       |
| ***3*** | 3       | 4       | 5       | 6       |

Let's begin with an initial array where the border elements are solved, but the elements in the middle are set to 0.

|         | ***0*** | ***1*** | ***2*** | ***3*** |
|---------|---------|---------|---------|---------|
| ***0*** | 0       | 1       | 2       | 3       |        
| ***1*** | 1       | *0*     | *0*     | 4       |
| ***2*** | 2       | *0*     | *0*     | 5       |
| ***3*** | 3       | 4       | 5       | 6       |

In our first iteration, we replace the 0s with the average of its 4 neighbours. For example, in (1,1), we replace 0 with (1+1+0+0)/4 = 0.5.

After the first iteration, the table looks like this:

|         | ***0*** | ***1*** | ***2*** | ***3*** |
|---------|---------|---------|---------|---------|
| ***0*** | 0       | 1       | 2       | 3       |        
| ***1*** | 1       | 0.5     | 1.5     | 4       |
| ***2*** | 2       | 1.5     | 2.5     | 5       |
| ***3*** | 3       | 4       | 5       | 6       |

The second iteration looks like this:

|         | ***0*** | ***1*** | ***2*** | ***3*** |
|---------|---------|---------|---------|---------|
| ***0*** | 0       | 1       | 2       | 3       |        
| ***1*** | 1       | 1.25    | 2.5     | 4       |
| ***2*** | 2       | 2.5     | 3.25    | 5       |
| ***3*** | 3       | 4       | 5       | 6       |

After the third iteration:

|         | ***0*** | ***1*** | ***2*** | ***3*** |
|---------|---------|---------|---------|---------|
| ***0*** | 0       | 1       | 2       | 3       |        
| ***1*** | 1       | 1.75    | 2.625   | 4       |
| ***2*** | 2       | 2.625   | 3.75    | 5       |
| ***3*** | 3       | 4       | 5       | 6       |


Fourth iteration:

|         | ***0*** | ***1*** | ***2*** | ***3*** |
|---------|---------|---------|---------|---------|
| ***0*** | 0       | 1       | 2       | 3       |        
| ***1*** | 1       | 1.81    | 2.875   | 4       |
| ***2*** | 2       | 2.875   | 3.8125  | 5       |
| ***3*** | 3       | 4       | 5       | 6       |

Fifth:

|         | ***0*** | ***1*** | ***2*** | ***3*** |
|---------|---------|---------|---------|---------|
| ***0*** | 0       | 1       | 2       | 3       |        
| ***1*** | 1       | 1.938   | 2.906   | 4       |
| ***2*** | 2       | 2.906   | 3.9375  | 5       |
| ***3*** | 3       | 4       | 5       | 6       |

After 5 iterations, the middle elements are already 0.1 precision away from the correct value.

# rusty-relaxation

This is a practise program to approximately solve a linear equation using the Jacobi relaxation method, first sequentially, and then concurrently.