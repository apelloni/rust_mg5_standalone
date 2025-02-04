#ifndef __MG5_CLASS__
#define __MG5_CLASS__

#include <cstdio>
#include <memory>
#include <string>
#include <vector>
#include <sstream>

#include "CPPProcess.h"

namespace MG5_NAMESPACE {
class MG5Integrand {
public:
  _CPPProcess process;
  // Constructor
  MG5Integrand();
  void init(const char *card_path);
  // void init();
  void set_momenta(const double *moms, const size_t size);
  double get_matrix_element();

  // Consts used for internal array limits
  size_t ninitial() const;
  size_t nexternal() const;
  size_t nprocesses() const;
  const double *get_masses() const;
  const std::string &get_name() const;

  // Get cout
  const std::string & read_cout();

private:
  std::vector<double *> p;
  std::vector<double> m;
  std::string name;
  // cout redirect
  std::stringstream buffer;
  std::streambuf *old;
  std::string cout;
};

std::unique_ptr<MG5Integrand> new_mg5_integrand();
} // namespace MG5_NAMESPACE
#endif
