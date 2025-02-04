#include "mg5_class.h"
//#include <iomanip>
//#include <iostream>
#include <cstdio>

namespace MG5_NAMESPACE {

// Return a unique pointer to the MG5Integrand class
std::unique_ptr<MG5Integrand> new_mg5_integrand() {
  return std::unique_ptr<MG5Integrand>(new MG5Integrand());
};

// Constructor
MG5Integrand::MG5Integrand() {}

// Initialise process with corresponding process_card
void MG5Integrand::init(const char *card_path) {
  process = _CPPProcess();
  name = process.name();
  printf("%s\n", card_path);
  process.initProc(card_path);
  // p.swap(std::vector<double *>(process.nexternal, {0.0, 0.0, 0.0, 0.0}))
  for (int i = 0; i < process.nexternal; i++) {
    double *pn = (double *)malloc(4 * sizeof(double));
    p.push_back(pn);
  }
}

// Set momenta used for the matrix element
void MG5Integrand::set_momenta(const double *moms, const size_t size) {
  for (size_t i = 0; i < size; i++)
    p[i / 4][i % 4] = moms[i];

   //for (size_t i = 0; i < process.nexternal; i++) {
   //    std::cout << "TMP: " << std::setw(4) << i + 1;
   //    std::cout << setiosflags(std::ios::scientific) << std::setw(14) << p[i][0];
   //    std::cout << setiosflags(std::ios::scientific) << std::setw(14) << p[i][1];
   //    std::cout << setiosflags(std::ios::scientific) << std::setw(14) << p[i][2];
   //    std::cout << setiosflags(std::ios::scientific) << std::setw(14) << p[i][3] << std::endl;
   //}
  process.setMomenta(p);
}

// Compute matrix element
double MG5Integrand::get_matrix_element() {
  // Evaluate matrix element
  process.sigmaKin();
  return process.getMatrixElements()[0];
}

// Return masses
const double *MG5Integrand::get_masses() const {
  return &process.getMasses()[0];
}

const std::string &MG5Integrand::get_name() const { return name; }

// Return internal constants for the array limits
size_t MG5Integrand::ninitial() const { return process.ninitial; }
size_t MG5Integrand::nexternal() const { return process.nexternal; }
size_t MG5Integrand::nprocesses() const { return process.nprocesses; }
} // namespace MG5_NAMESPACE
