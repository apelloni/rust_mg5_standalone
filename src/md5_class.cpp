#include <cstdio>
#include <iomanip>
#include <iostream>
#include <vector>

#include "md5_class.h"

// Return a unique pointer to the MD5Integrand class
std::unique_ptr<MD5Integrand> new_md5_integrand() {
  return std::unique_ptr<MD5Integrand>(new MD5Integrand());
};

// Constructor
MD5Integrand::MD5Integrand() {}

// Initialise process with corresponding process_card
void MD5Integrand::init(const char *card_path) {
  process = CPPProcess();
  printf("%s\n", card_path);
  process.initProc(card_path);
  // p.swap(std::vector<double *>(process.nexternal, {0.0, 0.0, 0.0, 0.0}))
  for (int i = 0; i < process.nexternal; i++) {
    double *pn = (double *)malloc(4 * sizeof(double));
    p.push_back(pn);
  }
}

// Set momenta used for the matrix element
void MD5Integrand::set_momenta(const double *moms, const size_t size) {
  for (size_t i = 0; i < size; i++)
    p[i / 4][i % 4] = moms[i];

  //for (size_t i = 0; i < process.nexternal; i++) {
  //  cout << "TMP: " << setw(4) << i + 1;
  //  cout << setiosflags(ios::scientific) << setw(14) << p[i][0];
  //  cout << setiosflags(ios::scientific) << setw(14) << p[i][1];
  //  cout << setiosflags(ios::scientific) << setw(14) << p[i][2];
  //  cout << setiosflags(ios::scientific) << setw(14) << p[i][3] << endl;
  //}
  process.setMomenta(p);
}

// Compute matrix element
double MD5Integrand::get_matrix_element() {
  // Evaluate matrix element
  process.sigmaKin();
  return process.getMatrixElements()[0];
}

// Return masses
const double *MD5Integrand::get_masses() const { return &process.getMasses()[0]; }

// Return internal constants for the array limits
size_t MD5Integrand::ninitial() const { return process.ninitial; }
size_t MD5Integrand::nexternal() const { return process.nexternal; }
size_t MD5Integrand::nprocesses() const { return process.nprocesses; }
