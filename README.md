# Advent of Code 2019

My solutions to [Advent of Code][aoc] 2019.

[aoc]: https://adventofcode.com/

## Running the tests

Given python3.8 + pip + virtualenv, create a virtualenv and install the
requirements, mainly [pytest][pytest] and my plugin [pytest-aoc][pytest-aoc]:

    virtualenv env
    env/bin/python3.8 -m pip install -r requirements.txt

Now run the tests:

    env/bin/pytest

⚠ If you're me: don't forget to put a cookie in the file `.cookie`.

[pytest]: https://pytest.org/
[pytest-aoc]: https://github.com/j0057/pytest-aoc
