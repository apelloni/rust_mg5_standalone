CC=g++ -O3

MG5=/home/andrea/Programs/MG5_aMC_v3_5_4/bin/mg5_aMC
PROCESS=standalone_uubar_aag

sources=src/CPPProcess.h src/Parameters_sm_ma.h src/read_slha.h

lib: lib/libmg5_class.so lib/libmodel_sm_ma.a


lib/libmodel_sm_ma.a:
	ln -sf ./../$(PROCESS)/lib/libmodel_sm_ma.a $@

$(PROCESS):
	./.venv/bin/python $(MG5) cards/$@.mg5

src/mg5_class.o: src/mg5_class.cpp $(PROCESS) $(sources)
	$(CC) -o $@ -c src/mg5_class.cpp -fPIC -Isrc/


lib/libmg5_class.so: src/mg5_class.o $(sources)
	mkdir -p lib/
	$(eval CPP_DIR=$(wildcard $(PROCESS)/SubProcesses/P1_Sigma_*/))
	make -C $(CPP_DIR)
	$(CC) --shared -fPIC -o lib/libmg5_class.so src/mg5_class.o $(CPP_DIR)/CPPProcess.o

 $(sources):
	$(eval CPPProcess=$(wildcard $(PROCESS)/SubProcesses/P1_Sigma_*/CPPProcess.h))
	cp $(CPPProcess) src/CPPProcess.h
	cp ./$(PROCESS)/src/Parameters_sm_ma.h src/Parameters_sm_ma.h
	cp ./$(PROCESS)/src/read_slha.h src/read_slha.h

clean:
	rm -f lib/*
	rm -f src/*.o
	rm -r src/CPPProcess.h src/Parameters_sm_ma.h src/read_slha.h
