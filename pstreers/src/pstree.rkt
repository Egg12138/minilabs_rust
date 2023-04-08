#lang scheme

(define (pstree pid)
  (define (get-proc)
    (call-with-input-file
     (string-append "/proc/" pid "/stat")
     (lambda (port)
       (let* ((stat (string-split (read-line port)))
              (ppid (list-ref stat 3)))
             (list (list pid ppid (string-remove-suffix")"
                         (string-remove-prefix "(" (list-ref stat 1))))))))))