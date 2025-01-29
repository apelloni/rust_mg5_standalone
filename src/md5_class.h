#include <cstdio>
#include <memory>
#include <vector>

#ifndef __MD5_CLASS__
#define __MD5_CLASS__
#include "CPPProcess.h"

class MD5Integrand {
public:
  CPPProcess process;
  // Constructor
  MD5Integrand();
  void init(const char *card_path);
  // void init();
  void set_momenta(const double *moms, const size_t size);
  double get_matrix_element();

  // Consts used for internal array limits
  size_t ninitial() const;
  size_t nexternal() const;
  size_t nprocesses() const;
  const double *get_masses() const;

private:
  std::vector<double *> p;
  std::vector<double> m;
};

std::unique_ptr<MD5Integrand> new_md5_integrand();

#endif
