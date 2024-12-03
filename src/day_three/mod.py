import re

pattern = r"mul\(\d{1,3},\d{1,3}\)|don't|do"
text = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"

matches = re.findall(pattern, text)
print(matches)
