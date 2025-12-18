#lang racket

(require data/applicative)
(require data/monad)
(require megaparsack
         megaparsack/text)

(define (parse-input-line line)
  (define space*/p (many/p space/p))
  (define space+/p (many+/p space/p))

  (define input-line/p
    (do (string/p "Card")
        space+/p
        [n <- integer/p]
        (char/p #\:)
        [win <- (many/p (try/p (do space*/p integer/p)))]
        space*/p
        (char/p #\|)
        [seq <- (many/p (try/p (do space*/p integer/p)))]
        (pure (list n win seq))))
  (parse-result! (parse-string input-line/p line)))

(define (part1 card)
  (match-define (list n win seq) card)
  (let* ([c (count (λ (e) (member e win)) seq)])
    (if (zero? c)
        0
        (expt 2 (sub1 c)))))

(define (part2 cards)
  (define/match (insert-sorted e list)
    [(_ (cons h t))
     #:when ((car h) . < . (car e))
     (cons h (insert-sorted e t))]
    [(_ (cons h t))
     #:when ((car h) . = . (car e))
     (cons (cons (car h) (+ (cdr h) (cdr e))) t)]
    [(_ _) (cons e list)])
  (for/fold ([cache '()]
             [res '()]
             #:result (reverse res))
            ([card cards])
    (match-define (list n win seq) card)
    (let* ([c (count (λ (e) (member e win)) seq)]
           [cache (dropf cache (λ (p) ((car p) . < . n)))]
           [w (add1 (apply + (map cdr cache)))])
      (values (insert-sorted (cons (+ n c) w) cache) (cons w res)))))

(module+ main
  (define input-lines (map parse-input-line (port->lines)))
  (displayln (apply + (map part1 input-lines)))
  (displayln (apply + (part2 input-lines))))

(module+ test
  (require rackunit)

  (define (split-lines str)
    (with-input-from-string str port->lines))

  (define sample-input
    (split-lines #<<EOF
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
EOF
                 ))

  (test-case "test part1"
    (check-equal? (map part1 (map parse-input-line sample-input)) '(8 2 2 1 0 0)))

  (test-case "test part2"
    (check-equal? (part2 (map parse-input-line sample-input)) '(1 2 4 8 14 1))))
