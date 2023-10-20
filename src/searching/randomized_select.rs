/*
 * Algorithm Design:
 * ------------------
 *
 * select(A[1..n], k) 
 * if |A| == 1: return A[1]
 *
 * s = some element of A
 * L = { a in A: a < s }
 * G = { a in A: a > s }
 * 
 * if k < |L|: return select(L, k) 
 * else if k == |L|: return L[n]
 * else: return select(A, k - |L| - 1)
 */

/*
 * Analysis on time complexity
 * ----------------------------
 *
 * Claim: 
 * -------
 *  - The expected running time of randomized_select is O(n).
 *  - The worst case is O(n^2) and best case is O(n).
 *
 * Proof:
 * -------
 * Let X be a random variable that represents the number of steps of algorithm.
 *  Consider the following chain of input sizes of recursive calls:
 *
 *      n_1 ---> n_1 ---> n_2 ---> n_3 ---> n_4 ---> n_5 ---> n_6
 *      |----------------------------|      |----------|
 *                  Phase 0                   Phase 1
 *
 *  where for each Phase j, the input size n_j is bounded by:
 *
 *                  (3/4)^{j-1} * n >= n_i > (3/4)^j * n.
 *
 * Let X_j be the number of steps taken by phase j, we can immediately see that:
 *                                             
 *                           X = Sum X_j, and  X_j <= M * N.
 *                                j            
 *
 *  Where 
 *   - M is the maximum number of steps taken by Phase j recursive call, that
 *      is, c(2/4)^j * n.
 *
 *   - N is the number of recursive calls in Phase j, which is another random 
 *      variable, call it Y_j.
 *      - Notice that we can choose a "good" splitter s, so that the number of 
 *         recursive calls in each phase is not greater than 2.
 *
 *  Therefore, 
 *                  E(X_j) <= E( c(2/4)^j * n * Y_j )
 *                         =  c(3/4)^j * n * E(Y_j)
 *                         =  2cn(3/4)^j
 *  It follows that:
 *                  E(X) =  E( Sum X_j )
 *                       =  Sum E(X_j)
 *                       =  2cn Sum (3/4)^j
 *                       =  8cn 
 *                       in Θ(n)
 */
