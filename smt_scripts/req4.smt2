(set-logic UFLRA)

; model phi as arbitrary predicate over reals
(declare-fun phi (Real) Bool)

; H[1,6] phi
(define-fun a ((t Real)) Bool
    (forall ((tp Real)) (=> (and (<= (- t 6) tp) (<= tp (- t 1)))               ; ∀ t' ∈ [t-6, t-1]
            (phi tp))))

; H[0,1] O[0,1] H[0,5] phi
(define-fun b ((t Real)) Bool
    (forall ((tp Real)) (=> (and (<= (- t 1) tp) (<= tp t))                     ; ∀ t' ∈ [t-1, t]
        (exists ((tpp Real)) (and (and (<= (- tp 1) tpp) (<= tpp tp))           ; ∃ t'' ∈ [t'-1, t']
            (forall ((tppp Real)) (=> (and (<= (- tpp 5) tppp) (<= tppp tpp))   ; ∀ t''' ∈ [t''-5, t'']
                (phi tppp))))))))

; negation of the formulae being equivalent
; so this program evaluating to `unsat` means that they ARE equivalent
(assert (exists ((t Real))
    (xor
        (a t)
        (b t))))

(check-sat)
