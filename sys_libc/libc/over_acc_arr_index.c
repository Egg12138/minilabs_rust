#include <stdio.h>

void degmm(int n, double *C) {
    for (int i = 0; i < n ; ++i) {
        for (int j = 0; j < n; ++j) {
                double cij = C[i+j*n];
                if (cij == C[i][j]) 
                    printf("%f = %f\n", cij, cij);
        }
    }
}


int
main()
{
        double C[] = {1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7, 8.8};
        degmm(2,  (double *)C);
        return 0;
}
