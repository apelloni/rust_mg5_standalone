#include <cstdio>
#include <iomanip>
#include <iostream>
#include <vector>

#include "md5_class.h"

// Constructor
MD5Integrand::MD5Integrand() {}
void MD5Integrand::init(){
  //printf("%s\n", card_path);
  process.initProc("/home/andrea/github/threshold_subtraction_mc/tools/madgraph/uubar_aag/Cards/param_card.dat");
  // p.swap(std::vector<double *>(process.nexternal, {0.0, 0.0, 0.0, 0.0}))
  for (size_t i = 0; i < process.nexternal; i++) {
    double *pn = (double *)malloc(4 * sizeof(double));
    p.push_back(pn);
  }
}
//void MD5Integrand::init(const char *card_path) {
//  process = CPPProcess();
//  printf("%s\n", card_path);
//  process.initProc(card_path);
//  // p.swap(std::vector<double *>(process.nexternal, {0.0, 0.0, 0.0, 0.0}))
//  for (int i = 0; i < process.nexternal; i++) {
//    double *pn = (double *)malloc(4 * sizeof(double));
//    p.push_back(pn);
//  }
//}

void MD5Integrand::set_momenta(double const *data, std::size_t size) {
  for (size_t i = 0; i < size; i++)
    for (size_t j = 0; j < 4; j++)
      p[i][j] = data[i * 4 + j];
  for (size_t i = 0; i < process.nexternal; i++) {
    cout << "TMP: " << setw(4) << i + 1;
    cout << setiosflags(ios::scientific) << setw(14) << p[i][0];
    cout << setiosflags(ios::scientific) << setw(14) << p[i][1];
    cout << setiosflags(ios::scientific) << setw(14) << p[i][2];
    cout << setiosflags(ios::scientific) << setw(14) << p[i][3] << endl;
  }
  process.setMomenta(p);
}

std::unique_ptr<MD5Integrand> new_md5_integrand() {
  return std::unique_ptr<MD5Integrand>(new MD5Integrand());
};

double MD5Integrand::get_matrix_element() {
  // Evaluate matrix element
  process.sigmaKin();
  return process.getMatrixElements()[0];
}

const double *MD5Integrand::get_masses() { return &process.getMasses()[0]; }

int MD5Integrand::nexternal() { return process.nexternal; }
int MD5Integrand::ninitial() { return process.ninitial; }
