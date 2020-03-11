#!/usr/bin/python3

"""
Make sure you have built the project, renamed `libCheeseShop.so` to
`CheeseShop.so`, and have that file in the same directory as this .py file.

Note that /// comments in Rust will translate to Python help() documentation:
>>> help(CheeseShop.do_something)
Help on built-in function do_something:

do_something(...)
    Does something completely different by returning a Python `List[str]`.
"""

from CheeseShop import *




# MODULE-LEVEL FUNCTIONS, not particularly interesting
assert do_something() == ['And', 'now', 'for', 'something', 'completely', 'different']

for movie, year in movies():
    print("%s was released in %d" % (movie, year))


## CHEESE SHOP
# How many cheeses do we not have?
assert CheeseShop.unavailable_cheese_count() == 45


cs = CheeseShop(is_hungry=True)
assert str(cs) == 'CheeseShop(stock=0)'

cs.camelot = "silly" # prints 'Setting self.camelot = "silly"'
try:
   foo = cs.does_not_exist
except ValueError as e:
    print(e) # "Attribute not found. Move along."

# What kinds of chesee do we have?
assert not cs.has_cheese("cheddar") # prints "We have no cheddar"
assert not cs.has_cheese("greek feta") # prints "We have no cheddar"
assert not cs.has_cheese() # prints "No cheese whatsoever."

# Run the front of the shop
print(cs.respond_to_client("camembert")) # "Oh! The cat's eaten it.cheddar"
print(cs.respond_to_client("some cheese whose name I just made up")) # "No"