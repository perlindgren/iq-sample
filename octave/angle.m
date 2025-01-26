% returns normalized -1..1 angle
function w = angle (i0, q0, i1, q1)
    w = atan2(i0 - i1, q0 - q1) / (2 * pi);
endfunction
