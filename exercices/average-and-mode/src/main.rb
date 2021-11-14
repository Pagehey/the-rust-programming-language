grades = [12, 14, 14, 13, 15, 17, 20, 19, 14, 13, 11, 9]
average = grades.sum.fdiv(grades.length).round(2)
max_grade, occurence = grades.tally.max_by(&:last)

puts "average is #{average}"
puts "grade #{max_grade} appears #{occurence} times."
