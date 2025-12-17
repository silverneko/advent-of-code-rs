#lang racket

(define (calc a b)
  ((a . * . 10) . + . b))

(define (digit? c)
  (and (char-numeric? c) (string->number (string c))))

(define (digit-text-prefix? str)
  (define number-patterns
    #[(1 . "one")
      (2 . "two")
      (3 . "three")
      (4 . "four")
      (5 . "five")
      (6 . "six")
      (7 . "seven")
      (8 . "eight")
      (9 . "nine")])
  (for/or ([np number-patterns])
    (match-let ([(cons d pat) np])
      (and (string-prefix? str pat) d))))

(define (part1 str)
  (let* ([digits (filter-map digit? (string->list str))]) (calc (first digits) (last digits))))

(define (part2 str)
  (let* ([digits (filter-map (λ (idx)
                               (or (digit? (string-ref str idx))
                                   (digit-text-prefix? (substring str idx))))
                             (range (string-length str)))])
    (calc (first digits) (last digits))))

(module+ main
  (define input-lines (port->lines))
  (displayln (apply + (map part1 input-lines)))
  (displayln (apply + (map part2 input-lines))))

(module+ test
  (require rackunit)

  (define (split-lines str)
    (with-input-from-string str port->lines))

  (define sample-input1
    (split-lines #<<EOF
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
EOF
                 ))

  (define sample-input2
    (split-lines #<<END
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
END
                 ))

  (test-case "test part1"
    (check-equal? (map part1 sample-input1) '(12 38 15 77)))

  (test-case "test part2"
    (check-equal? (map part2 sample-input2) '(29 83 13 24 42 14 76)))

  (test-case "test part2 edge case 81"
    (check-equal? (part2 "eightwo1") 81))

  (test-case "test part2 edge case 21"
    (check-equal? (part2 "2twone") 21)))
