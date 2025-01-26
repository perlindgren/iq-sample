function w = sample (t, p_4, ft)
    i0 = get_sample(t, ft);
    q0 = get_sample(t + p_4, ft);
    i1 = get_sample(t + 2 * p_4, ft);
    q1 = get_sample(t + 3 * p_4, ft);
    w = angle(i0, i1, q0, q1)

endfunction
