
def fuel(mass):
    return mass // 3 - 2

def full_fuel(mass):
    result = 0
    while mass > 3:
        mass = fuel(mass)
        result += mass if mass > 0 else 0
    return result

def day01a(masses):
    return sum(fuel(mass) for mass in masses)

def day01b(masses):
    return sum(full_fuel(mass) for mass in masses)

def test_01_ex1(): assert fuel(12) == 2
def test_01_ex2(): assert fuel(14) == 2
def test_01_ex3(): assert fuel(1969) == 654
def test_01_ex4(): assert fuel(100756) == 33583

def test_01_ex5(): assert full_fuel(14) == 2
def test_01_ex6(): assert full_fuel(1969) == 966
def test_01_ex7(): assert full_fuel(100756) == 50346

def test_01a(day01_numbers): assert day01a(day01_numbers) == 3374289
def test_01b(day01_numbers): assert day01b(day01_numbers) == 5058559
