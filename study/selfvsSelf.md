# What's the difference between self and Self?

`self` when used as first method argument, is a shorthand for `self: Self`. 
There are also `&self`, which is equivalent to `self: &Self`, 
and `&mut self`, which is equivalent to self: `&mut Self`.

`Self` in method arguments is syntactic sugar for the **receiving type** of the method 
(i.e. the type whose impl this method is in). 
This also allows for generic types without too much repetition.
