#!/usr/bin/env python3

def read_data():
    with open('./input', 'r') as f:
        raw_array = f.read().split("\n\n") 
        parsed_array = []
        for element in raw_array:
            element = element.split("\n")
            for i,_ in enumerate(element): element[i] = element[i].strip().split(':')
            parsed_array.append(element)
    return parsed_array

class Monkey:
  def __init__(self,name,items,operation,test,if_true,if_false,inspections):
    self.name = name
    self.items = items
    self.operation = operation
    self.test = test
    self.if_true = if_true
    self.if_false = if_false
    self.inspections = inspections

def build_monkey_classes(parsed_array):
    monkey_array = []
    for monkey in parsed_array:
        new_monkey = Monkey(monkey[0][0].split(' ')[1],monkey[1][1].split(','),monkey[2][1].split(' ')[4:6],monkey[3][1].split(' ')[3],monkey[4][1].split(' ')[4],monkey[5][1].split(' ')[4],0)
        monkey_array.append(new_monkey)
    return monkey_array
    
def find_business(monkey_array,cycles,divide):
    inspection_aray = []
    common_denominator = 1
    for monkey in monkey_array:
       common_denominator*=int(monkey.test)
    for _ in range(cycles):
        for monkey in monkey_array:
            for _ in range(len(monkey.items)):
                item_score = int(monkey.items.pop(0))
                if monkey.operation[1] == 'old': operation_number = item_score
                else: operation_number = int(monkey.operation[1]) 
                if monkey.operation[0] == '*': worry_level = operation_number*item_score
                elif monkey.operation[0] == '+': worry_level = operation_number+item_score
                if divide: worry_level = int(worry_level/3)
                if worry_level%int(monkey.test) == 0:monkey_array[int(monkey.if_true)].items.append(str(worry_level%common_denominator))
                else: monkey_array[int(monkey.if_false)].items.append(str(worry_level%common_denominator))
                monkey.inspections+=1
    for monkey in monkey_array:
        inspection_aray.append(monkey.inspections)
    inspection_aray.sort()
    business_score = inspection_aray[-1]*inspection_aray[-2]
    return business_score

if __name__ == "__main__":
    parsed_array = read_data()
    cycles,divide = 20,True
    monkey_array = build_monkey_classes(parsed_array)
    first_score = find_business(monkey_array,cycles,divide)
    cycles,divide = 10000,False
    monkey_array = build_monkey_classes(parsed_array)
    second_score = find_business(monkey_array,cycles,divide)
    print("First Score: ", first_score,"Second Score: ",second_score )