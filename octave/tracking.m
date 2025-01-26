function r = tracking (f, ft)

    % assumed period
    p = 1 / f
    p_4 = p / 4;

    t = 0.0;
    w0 = sample(t, p_4, ft)
    t = p;
    it = 0;

    while (1)
        it += 1
        w1 = sample(t, p_4, ft)
        diff = w1 - w0
        %     // println!("p_4 {} w0 {}, w1 {}, diff {}", p_4, w0, w1, diff);

        t = t + p;
        f = 1 / p

        if (abs(diff) < 0.0001) || (f > 400)
            break
        else
            p_4 -= 0.5 * p_4 * diff;
            p = p_4 * 4;
            w0 = w1;
        endif

    endwhile

    r = f

endfunction
