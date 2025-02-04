CC=g++ -O3

MG5=/home/andrea/Programs/MG5_aMC_v3_5_4/bin/mg5_aMC

MODEL=sm_ma
PROCESSES=sm_ma_uux_aa sm_ma_uux_aag sm_ma_uux_aaddx
PROCESSES_DIR=standalone_sm_ma

sources=src/cpp/Parameters_sm_ma.h src/cpp/read_slha.h

all: $(patsubst %, src/cpp/CPP_%.h, $(PROCESSES)) \
	 $(patsubst %,lib/libmg5_%.so , $(PROCESSES)) \
	 lib/libmodel_sm_ma.a

lib/librmg5.so: $(patsubst %, src/cpp/mg5_%.o, $(PROCESSES))
	mkdir -p lib/
	$(eval CPP_SRC=$(wildcard $(PROCESSES_DIR)/SubProcesses/P*_Sigma_*/CPPProcess.o))
	$(CC) --shared -fPIC -o $@ $^ $(CPP_SRC)

lib/librmg5.a: $(patsubst %, src/cpp/mg5_%.o, $(PROCESSES))
	$(eval CPP_SRC=$(wildcard $(PROCESSES_DIR)/SubProcesses/P*_Sigma_*/CPPProcess.o))
	ar rcs -o $@ $^ $(CPP_SRC)

lib/libmodel_sm_ma.a:
	ln -sf ./../$(PROCESSES_DIR)/lib/libmodel_sm_ma.a $@

$(PROCESSES_DIR):
	./.venv/bin/python $(MG5) cards/$@.mg5

src/cpp/mg5_%.o: src/mg5_class.cpp $(PROCESSES_DIR) src/cpp/CPP_%.h  $(sources)
	mkdir -p src/cpp/
	sed -e "s/MG5_NAMESPACE/MG5_$*/g" \
		-e "s/mg5_class.h/mg5_$*.h/" \
		-e "s/_CPPProcess/CPP_$*/" src/mg5_class.cpp > src/cpp/mg5_$*.cpp
	sed -e "s/MG5_NAMESPACE/MG5_$*/g" \
		-e "s/CPPProcess.h/CPP_$*.h/" \
		-e "s/_CPPProcess/CPP_$*/" src/mg5_class.h > src/cpp/mg5_$*.h
	$(CC) -o $@ -c src/cpp/mg5_$*.cpp -fPIC -Isrc/


lib/libmg5_%.so: src/cpp/mg5_%.o $(sources) src/cpp/CPP_%.h
	mkdir -p lib/
	$(eval CPP_DIR=$(PROCESSES_DIR)/SubProcesses/P*_Sigma_$*/)
	make -C $(CPP_DIR) CPPProcess.o
	$(CC) --shared -fPIC -o $@ src/cpp/mg5_$*.o $(CPP_DIR)/CPPProcess.o

src/cpp/CPP_%.h:
	mkdir -p src/cpp/
	$(eval CPP_DIR=$(PROCESSES_DIR)/SubProcesses/P*_Sigma_$*/)
	sed -i "s/CPPProcess\([^.]\)/CPP_$*\1/g" $(CPP_DIR)/CPPProcess.cc
	sed -i "s/CPPProcess/CPP_$*/g" $(CPP_DIR)/CPPProcess.h
	cp ./$(PROCESSES_DIR)/SubProcesses/P*_Sigma_$*/CPPProcess.h $@

 $(sources):
	mkdir -p src/cpp/
	cp ./$(PROCESSES_DIR)/src/Parameters_sm_ma.h src/cpp/.
	cp ./$(PROCESSES_DIR)/src/read_slha.h src/cpp/.

clean:
	rm -f lib/*
	rm -f src/cpp/*
	#rm -f src/cpp/*.o
	#rm -r src/cpp/CPP_* src/cpp/Parameters_sm_ma* src/cpp/read_slha*
