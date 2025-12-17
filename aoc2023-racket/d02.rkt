#lang racket

(require data/applicative)
(require data/monad)
(require megaparsack
         megaparsack/text)

(define space*/p (many/p space/p))
(define space+/p (many+/p space/p))

(struct game-round (red blue green) #:prefab)

(define game-round/p
  (do [s
       <-
       (many+/p #:sep (char/p #\,)
                (do space*/p
                    [t <- integer/p]
                    space+/p
                    [c <- (many+/p letter/p)]
                    (pure (cons (list->string c) t))))]
      (let* ([h (make-hash s)]
             [red (hash-ref h "red" 0)]
             [blue (hash-ref h "blue" 0)]
             [green (hash-ref h "green" 0)])
        (pure (game-round red blue green)))))

(define input-line/p
  (do (string/p "Game")
      space+/p
      [n <- integer/p]
      (char/p #\:)
      [rounds <- (many/p game-round/p #:sep (char/p #\;))]
      (pure (cons n rounds))))

(define (parse-input-line line)
  (parse-result! (parse-string input-line/p line)))

(define (fold-game-rounds rounds)
  (for/fold ([acc (game-round 0 0 0)]) ([r rounds])
    (let ([red (max (game-round-red acc) (game-round-red r))]
          [blue (max (game-round-blue acc) (game-round-blue r))]
          [green (max (game-round-green acc) (game-round-green r))])
      (game-round red blue green))))

(define (part1 game)
  (match-define (game-round red blue green) (fold-game-rounds (cdr game)))
  (and (<= red 12) (<= green 13) (<= blue 14) (car game)))

(define (part2 game)
  (match-define (game-round red blue green) (fold-game-rounds (cdr game)))
  (* red blue green))

(module+ main
  (define input-lines (map parse-input-line (port->lines)))
  (displayln (apply + (filter-map part1 input-lines)))
  (displayln (apply + (map part2 input-lines))))

(module+ test
  (require rackunit)

  (define (split-lines str)
    (with-input-from-string str port->lines))

  (define sample-input
    (split-lines #<<EOF
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
EOF
                 ))

  (test-case "test part1"
    (check-equal? (map part1 (map parse-input-line sample-input)) '(1 2 #f #f 5)))

  (test-case "test part2"
    (check-equal? (map part2 (map parse-input-line sample-input)) '(48 12 1560 630 36))))
