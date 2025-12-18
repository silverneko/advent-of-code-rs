#lang racket

(define (parse-input)
  (for/vector ([l (in-lines)])
    l))

(define (grid-get? grid x y)
  (and (< -1 y (vector-length grid))
       (< -1 x (string-length (vector-ref grid y)))
       (string-ref (vector-ref grid y) x)))

(define (check-symbols grid x y)
  (for*/list ([dx '(-1 0 1)]
              [dy '(-1 0 1)]
              #:do [(define-values (nx ny) (values (+ x dx) (+ y dy)))]
              #:do [(define c (grid-get? grid nx ny))]
              #:when (and c (not (char-numeric? c)) (not (char=? c #\.)) c))
    (list c nx ny)))

(define (dig grid [x 0] [y 0])
  (define c (grid-get? grid x y))
  (cond
    [(>= y (vector-length grid)) '()]
    [(not c) (dig grid 0 (add1 y))]
    [(not (char-numeric? c)) (dig grid (add1 x) y)]
    [else
     (define line (vector-ref grid y))
     (define end
       (let loop ([end (add1 x)])
         (if (and (< end (string-length line)) (char-numeric? (string-ref line end)))
             (loop (add1 end))
             end)))
     (define num (string->number (substring line x end)))
     (define symbols
       (remove-duplicates (for*/list ([i (in-range x end)]
                                      [n (check-symbols grid i y)])
                            n)))
     (if (empty? symbols)
         (dig grid end y)
         (cons (cons num symbols) (dig grid end y)))]))

(define (part2 nums)
  (define stars
    (remove-duplicates (for*/list ([syms (map cdr nums)]
                                   [s syms]
                                   #:when (char=? (car s) #\*))
                         s)))
  (for/list ([star stars]
             #:do [(define ns (filter-map (λ (e) (and (member star (cdr e)) (car e))) nums))]
             #:when (= 2 (length ns)))
    ns))

(module+ main
  (define grid (parse-input))
  (define nums (dig grid))
  (displayln (apply + (map car nums)))
  (displayln (apply + (map (λ (p) (apply * p)) (part2 nums)))))

(module+ test
  (require rackunit)

  (define sample-input
    (let ([input #<<EOF
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
EOF
           ])
      (with-input-from-string input parse-input)))

  (test-case "test part1"
    (check-equal? (map car (dig sample-input)) '(467 35 633 617 592 755 664 598)))

  (test-case "test part2"
    (check-equal? (part2 (dig sample-input)) '((467 35) (755 598)))))
