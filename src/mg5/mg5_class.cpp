#include "mg5_class.h"
// #include <iomanip>
#include <cstdio>
#include <iostream>
#include <sstream>

namespace MG5_NAMESPACE {

// Return a unique pointer to the MG5Integrand class
std::unique_ptr<MG5Integrand> new_mg5_integrand() {
  return std::unique_ptr<MG5Integrand>(new MG5Integrand());
};

// Constructor
MG5Integrand::MG5Integrand() {
  // Preparecontainers
  c = {0.0, 0.0};
  // Redirect stdout to buffer
  old = std::cout.rdbuf(buffer.rdbuf());
}

// Deconstructor
MG5Integrand::~MG5Integrand() {
  for (int i = 0; i < process.nexternal; i++) {
    free(p[i]);
  }
}

// Initialise process with corresponding process_card
void MG5Integrand::init(const char *card_path) {
  // Create CPPProcess and set card parameters
  process = _CPPProcess();
  name = process.name();
  // printf("%s\n", card_path);
  process.initProc(card_path);
  parameters = process.get_pars();
  // p.swap(std::vector<double *>(process.nexternal, {0.0, 0.0, 0.0, 0.0}))
  for (int i = 0; i < process.nexternal; i++) {
    double *pn = (double *)malloc(400 * sizeof(double));
    p.push_back(pn);
  }
}

// Set momenta used for the matrix element
void MG5Integrand::set_momenta(const double *moms, const size_t size) {
  for (size_t i = 0; i < size; i++)
    p[i / 4][i % 4] = moms[i];

  // for (size_t i = 0; i < process.nexternal; i++) {
  //     std::cout << "TMP: " << std::setw(4) << i + 1;
  //     std::cout << setiosflags(std::ios::scientific) << std::setw(14) <<
  //     p[i][0]; std::cout << setiosflags(std::ios::scientific) <<
  //     std::setw(14) << p[i][1]; std::cout <<
  //     setiosflags(std::ios::scientific) << std::setw(14) << p[i][2];
  //     std::cout << setiosflags(std::ios::scientific) << std::setw(14) <<
  //     p[i][3] << std::endl;
  // }
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

const std::string &MG5Integrand::read_cout() {
  // Reset stdout
  // std::cout.rdbuf(old);
  cout = buffer.str();
  buffer.str("");
  buffer.clear();
  return cout;
}

// Obtain parameters
//
// Model parameters independent of aS
double MG5Integrand::get_mdl_WH() const { return parameters->mdl_WH; }
double MG5Integrand::get_mdl_WW() const { return parameters->mdl_WW; }
double MG5Integrand::get_mdl_WZ() const { return parameters->mdl_WZ; }
double MG5Integrand::get_mdl_WT() const { return parameters->mdl_WT; }
double MG5Integrand::get_mdl_ymtau() const { return parameters->mdl_ymtau; }
double MG5Integrand::get_mdl_ymt() const { return parameters->mdl_ymt; }
double MG5Integrand::get_mdl_ymb() const { return parameters->mdl_ymb; }
double MG5Integrand::get_aS() const { return parameters->aS; }
// double MG5Integrand::get_aS() const { return parameters->aS; }
double MG5Integrand::get_mdl_Gf() const { return parameters->mdl_Gf; }
double MG5Integrand::get_aEWM1() const { return parameters->aEWM1; }
double MG5Integrand::get_mdl_MH() const { return parameters->mdl_MH; }
double MG5Integrand::get_mdl_MZ() const { return parameters->mdl_MZ; }
double MG5Integrand::get_mdl_MTA() const { return parameters->mdl_MTA; }
double MG5Integrand::get_mdl_MT() const { return parameters->mdl_MT; }
double MG5Integrand::get_mdl_MB() const { return parameters->mdl_MB; }
double MG5Integrand::get_mdl_CKM3x3() const { return parameters->mdl_CKM3x3; }
double MG5Integrand::get_mdl_conjg_CKM1x1() const {
  return parameters->mdl_conjg__CKM1x1;
}
double MG5Integrand::get_mdl_conjg_CKM3x3() const {
  return parameters->mdl_conjg__CKM3x3;
}
double MG5Integrand::get_mdl_MZ_exp_2() const {
  return parameters->mdl_MZ__exp__2;
}
double MG5Integrand::get_mdl_MZ_exp_4() const {
  return parameters->mdl_MZ__exp__4;
}
double MG5Integrand::get_mdl_sqrt_2() const { return parameters->mdl_sqrt__2; }
double MG5Integrand::get_mdl_MH_exp_2() const {
  return parameters->mdl_MH__exp__2;
}
double MG5Integrand::get_mdl_aEW() const { return parameters->mdl_aEW; }
double MG5Integrand::get_mdl_MW() const { return parameters->mdl_MW; }
double MG5Integrand::get_mdl_sqrt_aEW() const {
  return parameters->mdl_sqrt__aEW;
}
double MG5Integrand::get_mdl_ee() const { return parameters->mdl_ee; }
double MG5Integrand::get_mdl_MW_exp_2() const {
  return parameters->mdl_MW__exp__2;
}
double MG5Integrand::get_mdl_sw2() const { return parameters->mdl_sw2; }
double MG5Integrand::get_mdl_cw() const { return parameters->mdl_cw; }
double MG5Integrand::get_mdl_sqrt_sw2() const {
  return parameters->mdl_sqrt__sw2;
}
double MG5Integrand::get_mdl_sw() const { return parameters->mdl_sw; }
double MG5Integrand::get_mdl_g1() const { return parameters->mdl_g1; }
double MG5Integrand::get_mdl_gw() const { return parameters->mdl_gw; }
double MG5Integrand::get_mdl_vev() const { return parameters->mdl_vev; }
double MG5Integrand::get_mdl_vev_exp_2() const {
  return parameters->mdl_vev__exp__2;
}
double MG5Integrand::get_mdl_lam() const { return parameters->mdl_lam; }
double MG5Integrand::get_mdl_yb() const { return parameters->mdl_yb; }
double MG5Integrand::get_mdl_yt() const { return parameters->mdl_yt; }
double MG5Integrand::get_mdl_ytau() const { return parameters->mdl_ytau; }
double MG5Integrand::get_mdl_muH() const { return parameters->mdl_muH; }
double MG5Integrand::get_mdl_ee_exp_2() const {
  return parameters->mdl_ee__exp__2;
}
double MG5Integrand::get_mdl_sw_exp_2() const {
  return parameters->mdl_sw__exp__2;
}
double MG5Integrand::get_mdl_cw_exp_2() const {
  return parameters->mdl_cw__exp__2;
}
const std::vector<double> &MG5Integrand::get_mdl_complexi() {
  c[0] = parameters->mdl_complexi.real();
  c[1] = parameters->mdl_complexi.imag();
  return c;
}
const std::vector<double> &MG5Integrand::get_mdl_I1x33() {
  c[0] = parameters->mdl_I1x33.real();
  c[1] = parameters->mdl_I1x33.imag();
  return c;
}
const std::vector<double> &MG5Integrand::get_mdl_I2x33() {
  c[0] = parameters->mdl_I2x33.real();
  c[1] = parameters->mdl_I2x33.imag();
  return c;
}
const std::vector<double> &MG5Integrand::get_mdl_I3x33() {
  c[0] = parameters->mdl_I3x33.real();
  c[1] = parameters->mdl_I3x33.imag();
  return c;
}
const std::vector<double> &MG5Integrand::get_mdl_I4x33() {
  c[0] = parameters->mdl_I4x33.real();
  c[1] = parameters->mdl_I4x33.imag();
  return c;
}

// Model parameters dependent on aS
//
double MG5Integrand::get_mdl_sqrt_aS() const {
  return parameters->mdl_sqrt__aS;
}
double MG5Integrand::get_G() const { return parameters->G; }
double MG5Integrand::get_mdl_G_exp_2() const {
  return parameters->mdl_G__exp__2;
}

// Model couplings independent of aS
const std::vector<double> &MG5Integrand::get_GC_2() {
  c[0] = parameters->GC_2.real();
  c[1] = parameters->GC_2.imag();
  return c;
}

// Model couplings dependent on aS
const std::vector<double> &MG5Integrand::get_GC_11() {
  c[0] = parameters->GC_11.real();
  c[1] = parameters->GC_11.imag();
  return c;
}

} // namespace MG5_NAMESPACE
