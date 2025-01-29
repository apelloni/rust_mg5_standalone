#include <cstdio>
#include <vector>
#include <memory>


#ifndef __MD5_CLASS__
#define __MD5_CLASS__
#include "CPPProcess.h"

class MD5Integrand {
public:
  CPPProcess process;
  // Constructor
  MD5Integrand();
  //void init(const char *card_path);
  void init();
  void set_momenta(double const *data, std::size_t size);
  double get_matrix_element();
  int nexternal();
  int ninitial();
  const double * get_masses();

private:
  std::vector<double *> p;
  std::vector<double>  m;
};

std::unique_ptr<MD5Integrand> new_md5_integrand();

#endif
