#include <cassert>
#include <iostream>

#include <blis/cblas.h>
#include <sys/time.h>

#include <immintrin.h>
#include <xmmintrin.h>

const int m = 128;
const int n = 256;
const int k = 1024;

const int iter = 10;

double now_in_sec() {
  struct timeval tv;
  gettimeofday(&tv, NULL);
  return tv.tv_sec + tv.tv_usec * 1e-6;
}

void myblas_sgemm_1(int m, int n, int k, const float *a, int lda,
                    const float *b, int ldb, float *c, int ldc) {
  for (int i = 0; i < m; i++)
    for (int j = 0; j < n; j++) {
      float sum = 0.0;
#pragma clang loop vectorize(enable)
      for (int l = 0; l < k; l++)
        sum += a[i * lda + l] * b[l * ldb + j];
      c[i * ldc + j] = sum;
    }
}

void myblas_sgemm_2(int m, int n, int k, const float *a, int lda,
                    const float *b, int ldb, float *c, int ldc) {
  assert(m % 8 == 0);
  assert(n % 8 == 0);
  assert(k % 8 == 0);
  __m256 sum[8] = {_mm256_setzero_ps(), _mm256_setzero_ps(),
                   _mm256_setzero_ps(), _mm256_setzero_ps(),
                   _mm256_setzero_ps(), _mm256_setzero_ps(),
                   _mm256_setzero_ps(), _mm256_setzero_ps()};
  for (int i = 0; i < m; i += 8)
    for (int j = 0; j < n; j += 8) {

#pragma unroll
      for (int l = 0; l < 8; l++)
        sum[l] = _mm256_setzero_ps();
      for (int l = 0; l < k; l++) {
        _mm_prefetch((const char *)(b + (l + 0) * ldb + j), _MM_HINT_T0);
        _mm_prefetch((const char *)(b + (l + 1) * ldb + j), _MM_HINT_T0);
        _mm_prefetch((const char *)(b + (l + 2) * ldb + j), _MM_HINT_T0);
        _mm_prefetch((const char *)(b + (l + 3) * ldb + j), _MM_HINT_T0);
        __m256 as[8];
#pragma unroll
        for (int ll = 0; ll < 8; ll++)
          as[ll] = _mm256_broadcast_ss(a + (i + ll) * lda + l);
        __m256 bs = _mm256_loadu_ps(b + l * ldb + j);
#pragma unroll
        for (int ll = 0; ll < 8; ll++)
          sum[ll] = _mm256_fmadd_ps(as[ll], bs, sum[ll]);
      }
#pragma unroll
      for (int l = 0; l < 8; l++)
        _mm256_storeu_ps(c + (i + l) * ldc + j, sum[l]);
    }
}

void fill_random(float *x, int n) {
  for (int i = 0; i < n; i++)
    x[i] = (float)rand() / (float)RAND_MAX;
}

bool allclose(const float *x, const float *y, int n) {
  for (int i = 0; i < n; i++) {
    // std::cout << x[i] << " vs " << y[i] << std::endl;
    // std::cout << fabs(x[i] - y[i]) << std::endl;
    if (fabs(x[i] - y[i]) > 1e-3)
      return false;
  }
  return true;
}

int main() {
  float *x = (float *)calloc(m * k, sizeof(float));
  float *y = (float *)calloc(k * n, sizeof(float));
  float *cblas_z = (float *)calloc(m * n, sizeof(float));
  float *myblas_z = (float *)calloc(m * n, sizeof(float));

  fill_random(x, m * k);
  fill_random(y, k * n);

  for (int i = 0; i < iter; i++) {
    const int ave = 30;
    double cblas_elapsed = 0.0;
    double myblas_elapsed = 0.0;

    for (int j = 0; j < ave; j++) {
      const double cblas_start = now_in_sec();
      cblas_sgemm(CblasRowMajor, CblasNoTrans, CblasNoTrans, m, n, k, 1.0, x, k,
                  y, n, 0.0, cblas_z, n);
      cblas_elapsed += now_in_sec() - cblas_start;

      const double myblas_start = now_in_sec();
      myblas_sgemm_2(m, n, k, x, k, y, n, myblas_z, n);
      myblas_elapsed += now_in_sec() - myblas_start;

      assert(allclose(cblas_z, myblas_z, m * n));
    }

    std::cout << "[blis] " << (cblas_elapsed * 1000.0 / ave) << " [ms]"
              << std::endl;
    std::cout << "[mine] " << (myblas_elapsed * 1000.0 / ave) << " [ms]"
              << std::endl;
  }

  return 0;
}
