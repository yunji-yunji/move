use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::process::exit;
// use std::result::Result;
use std::borrow::Borrow;
use colored::Colorize;

use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{self, Result, Write};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use crate::file_format::{CompiledModule, empty_module, json_to_module, module_to_json, mutate_module};


#[derive(Debug, Clone)]
pub enum ReasonForStopping<T> {
    TestFailure(T),
    ExhaustedAllPossibleMutations,
    MaxIterationsReached,
    MaxDurationReached,
}

pub fn launch(test: Box<dyn Fn(&CompiledModule)>) {
// pub fn launch(test: Box<dyn Fn(&CompiledModule) + 'static>) {
    let mut fuzzer = Fuzzer::new(test);
    // <CompiledModule>::new(test);
    fuzzer.arguments_from_env_var();
    fuzzer.main_loop();

}


pub struct TestFailure {
    pub display: String,
    pub id: u64,
}
static mut TEST_FAILURE: Option<TestFailure> = None;
static mut DID_FIND_ANY_TEST_FAILURE: bool = false;


struct FuzzedInput<T> {
    value: T,
    // cache: Mut::Cache,
    // mutation_step: Mut::MutationStep,
    // generation: Generation,
}
impl<T> FuzzedInput<T> {
    fn new(val: T) -> Self {
        FuzzedInput {
            value : val,
        }
    }
}

pub fn from_data_to_CM(data: &[u8]) -> Option<CompiledModule> {
    serde_json::from_slice(data).ok()
}

pub fn from_CM_to_data(value: &CompiledModule) -> Vec<u8> {
    serde_json::to_vec(value).unwrap()
}


pub struct Fuzzer
// <T>
// where
//     T: Clone, // + 'static
{
    pub in_corpus_dir: PathBuf,
    pub exec_path_file: PathBuf,
    pub test: Box<dyn Fn(&CompiledModule)>,  // -> bool,
    pub test_failure: Option<TestFailure>,
}
// use move_bytecode_verifier::VerifierConfig;

impl Fuzzer{
// impl<T> Fuzzer<T>
// where
//     T: Clone,
// {
    // pub fn new() -> Self {
    pub fn new(test: Box<dyn Fn(&CompiledModule)>) -> Self {
        Self {
            in_corpus_dir: Default::default(),
            exec_path_file: Default::default(),
            // test: Default::default(),
            test: test,
            test_failure: None,
        }
    }

    pub fn main_loop(&mut self) {
        println!("---------------- MAIN LOOP -----------------");
        let mut initial_inputs : Vec<Vec<usize>> = Vec::new();
        // let mut initial_inputs = vec!();
        self.process_initial_inputs(&mut initial_inputs);
        println!("after process initial inputs {:?}", initial_inputs);
    }

    pub fn process_initial_inputs(&mut self, v: &mut Vec<Vec<usize>>) {
        /// read input corpus
        /// file -> CM
        /// 1. call read_input_corpus -> Result<Vec<Vec<u8>>>
        /// Read file and store it in vecstor as byte format(Vec<u8>)
        /// 2. byte format to CM format
        // self.target_struct.read_input
        println!("{}", "in process initial inputs".red().underline());
        let mut modules: Vec<CompiledModule> = self
            .read_input_corpus()
            .expect("READ_INPUT_FILE_ERROR")
            .into_iter()
            .filter_map(
                |value| {
                    /// value is Vec<u8>, value is data
                    let s = match String::from_utf8(value.clone()){
                        Ok(res) => res,
                        Err(e) => panic!("YJ Error = {:?}", e),
                    };
                    // println!("* value in string format {:?}", s);
                    let tmp = empty_module();
                    // let module = tmp.from_data(&value)?;
                    let module = from_data_to_CM(&value)?;
                    // let module = self.target_struct.from_data(&value)?;
                    // println!("value in byte format {:?}", self.target_struct.to_data(&module));
                    Some(module)
                }
            ).collect();
        println!("lenght of modules {:?}", modules.len());
/*
        // crate random CMs
        for i in 0..10 {
            // Result<Vec<Vec<u8>>>
            //     if let rand_module = arbitrary_module();{
        let s = match String::from_utf8(self.target_struct.to_data(&rand_module).clone()) {
            Ok(vv) => vv,
            Err(e) => panic!("error = {:?}", e),
        };
        println!("*  {:?}", s);
            // modules.push(rand_module);

            // } else {
                // break;
            // }
        }

 */

        // test and run
        let mut cnt = 1;
        for module in modules {
            // self.test_and_process_input::<CompiledModule>(v, module);
            self.test_and_process_input(v, module);

            let old_name = self.exec_path_file.to_str()
                .expect("old name to string");
            let new_name = format!("{}_init_{}", old_name.clone(), cnt);
            let _ = fs::rename(old_name, new_name);
            // println!("new name {}", new_name.green());

            cnt += 1;
        }


    }
    // fn test_and_process_input<T>(&mut self, paths: &mut Vec<Vec<usize>>, module: CompiledModule)
    fn test_and_process_input(&mut self,
                              paths: &mut Vec<Vec<usize>>, module: CompiledModule)
        // where
        //     T: Clone,

    /*-> Result<(), ReasonForStopping<T>> */
    {
        // let test = self.test;

        // check input is temporary or pool
        // no need now
        // let input = FuzzerState::<T, M>::get_input(input_idx, pool_storage).unwrap();

        std::panic::set_hook(Box::new(
                move |panic_info| {
                let mut hasher = DefaultHasher::new();
                panic_info.location().hash(&mut hasher);
                // unsafe {
                //     TEST_FAILURE = Some(TestFailure {
                //         display: format!("{}", panic_info),
                //         id: hasher.finish(),
                //     });
                // }
            },
        ));

        // same funtion
        // sensor_and_pool.start_recording();
        // fn start_recording(&mut self) {
        //     self.error = None;
        //     unsafe {
        //         TEST_FAILURE = None;
        //     }
        // }

        // unsafe {
        //     TEST_FAILURE = None;
        // }
        // let verifier_config : VerifierConfig= VerifierConfig::default();
        let fi = FuzzedInput::new(module);
        // let result = catch_unwind(|| (self.test)(fi.value.borrow()));
        // (&CompiledModule));
        let result = catch_unwind(AssertUnwindSafe(
            || (self.test)(fi.value.borrow()),
            // || (self.test)(&verifier_config, fi.value.borrow()),
        //     #[no_coverage]
        //         // || (self.test)(input.value.borrow()),
        //         || {
        //         // (self.test)(self.target_struct.to_data(&module).borrow())
        //         // (self.test)(module.to_data(&module).borrow())
        //         //     (self.test)(module.to_data(&module).borrow())
        //                 (self.test)(module.borrow())
        //     },
        ));
        println!("{}", "after result excuted result?".green().bold().underline());
        let _ = std::panic::take_hook();
        // let test_failure = match result {
        //     Ok(false) =>
        //     //     {
        //     //     true
        //     // }
        //     //     unsafe {
        //     //     TEST_FAILURE = Some(TestFailure {
        //     //         display: "test function returned false".to_string(),
        //     //         id: 0,
        //     //     });
        //         true
        //     // }
        //     ,
        //     Err(_) => {
        //         // the panic handler already changed the value of TEST_FAILURE
        //         // so we don't need to do anything
        //         true
        //     }
        //     Ok(true) => false,
        // };
        // if test_failure {
        //     println!("{}", "TEST FAILED".red());
        //     // unsafe {
        //     //     DID_FIND_ANY_TEST_FAILURE = true;
        //     // }
        // }
        // sensor_and_pool.stop_recording();
        // fn stop_recording(&mut self) {
        //     unsafe {
        //         self.error = TEST_FAILURE.clone();
        //     }
        // }



        // println!("before add new fuzzer yj");
        // add new fuzzer
        // if test_failure && self.state.settings.stop_after_first_failure {
        //     let serialized_input = serializer.to_data(&input.value);
        //     self.state
        //         .world
        //         .save_artifact(serialized_input, cplx, serializer.extension())
        //         .expect(SAVE_ARTIFACTS_ERROR);
        //     return Err(ReasonForStopping::TestFailure(input.value.clone()));
        // }

        // fuzzer_stats.total_number_of_runs += 1;

        // let input_id = PoolStorageIndex(pool_storage.next_slot());

        /*
                // read file
                // if the file is same in ant
                // println!("maybe after execution");
                // let file_name = "/home/y23kim/rust/test_progs/corpus/sub_dir/new_path";

                let arguments = std::env::var("FUZZCHECK_ARGS").unwrap();
                let args: Vec<&str> = arguments.split_whitespace().collect();
                // let in_corpus = PathBuf::from(args[1]);
                // let corpus= in_corpus.as_path();
                let file_name = args[1];
                println!("args and file name = {:?} {:?}", args, file_name);
        */
        let file_name = self.exec_path_file.clone();
        // println!("file name {:?}", file_name.clone());
        // let file  = fs::File::open(file_name);
        let contents = fs::read_to_string(file_name)
            .expect("read file in fuzzer.rs");

        let res_path: Vec<usize> = contents
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        // println!("in fuzzer.rs read result = {:?}, {:?}", contents, res_path);

        let contain = paths.iter().any(|inner_vec| inner_vec == &res_path);
        if contain {
            println!("{} {:?} \n{:?}","NOT interesting".blue(), paths, res_path);
        } else {
            paths.push(res_path);
            println!("{} {:?} ", "YES Interesting".red(), paths);

            // let deltas = sensor_and_pool.process(input_id, cplx);
            // println!("delta (outside) = {:?}", deltas);

            // if !deltas.is_empty() {
            // let add_ref_count = deltas.iter().fold(
            //     0,
            //     #[no_coverage]
            //         |acc, delta| if delta.add { acc + 1 } else { acc },
            // );
            /// not important
            // update_fuzzer_stats(fuzzer_stats, world);
            // let event = CorpusDelta::fuzzer_event(&deltas);
            // let content = if add_ref_count > 0 {
            //     serializer.to_data(&input.value)
            // } else {
            //     vec![]
            // };
            // let content = T.to_data();
            // let content = self.target_struct.to_data(&module);
            // let content = module.to_data(&module);
            // let content = from_CM_to_data(&module.clone());
            let content = from_CM_to_data(&fi.value);
            // let content = serializer.to_data(&input.value);
            /// update_corpus!
            &self
                .update_corpus(/*input_id,*/ content, true)
                .expect("UPDATE_CORPUS_ERROR");
            // world.report_event(event, Some((fuzzer_stats, sensor_and_pool.stats().as_ref())));
            // if add_ref_count > 0 {
            /*
                let generation = Generation(fuzzer_stats.total_number_of_runs);
                let input = input.new_source(mutator, generation);
                // check that the mutator's handling of the complexity is correct
                let serialised = String::from_utf8(serializer.to_data(&input.value)).unwrap();
/*
                assert!(
                    (input.complexity(mutator) - cplx).abs() < 0.01,
                    "The mutator used by the fuzz test does not evaluate the complexity of the test cases consistently.
                    This is a bug in the implementation of {}
                    =============

                    {serialised}

                    =============
                    ",
                    std::any::type_name::<M>()
                );


 */

                let mut subvalues: HashMap<TypeId, Vec<(*const dyn Any, f64)>> = HashMap::default();

                let mut act_on_subvalue = #[no_coverage]
                    |subvalue: &dyn Any, complexity| {
                    subvalues
                        .entry(subvalue.type_id())
                        .or_default()
                        .push((subvalue as *const _, complexity));
                };

                mutator.visit_subvalues(&input.value, &input.cache, &mut act_on_subvalue);
                let storage_idx_1 = pool_storage.next_slot();
                let subvalues = CrossoverSubValueProvider::new(
                    SubValueProviderId {
                        idx: storage_idx_1,
                        generation,
                    },
                    &input.value,
                    &input.cache,
                    mutator,
                );
                let stored_input = FuzzedInputAndSubValueProvider { input, subvalues };
                let storage_idx_2 = pool_storage.insert(stored_input, add_ref_count);
                assert_eq!(storage_idx_1, storage_idx_2);

             */

        }
        // for delta in deltas {
        //     for r in delta.remove {
        //         pool_storage.remove(r.0);
        //     }
        // }
        // }

        // Ok(())
    }


    pub fn arguments_from_env_var(&mut self) {

        let arg_in_corpus = std::env::var("IN_CORPUS").unwrap();
        self.in_corpus_dir = PathBuf::from(arg_in_corpus);
        // let in_corpus_path= self.in_corpus_dir.as_path();

        let arg_path_cov = std::env::var("PATH_COV").unwrap();
        self.exec_path_file = PathBuf::from(arg_path_cov);

        println!("===== arguemnts setting =====");
        println!("in_corpus {:?} path_cov {:?}", self.in_corpus_dir, self.exec_path_file);
        println!("===== ================= =====");
    }

    fn hash(&self, input: &[u8]) -> String {
        let mut hasher = DefaultHasher::new();
        input.hash(&mut hasher);
        let hash = hasher.finish();
        let hash = format!("{:x}", hash);
        hash
    }

    fn update_corpus(&mut self,
                     /*idx: PoolStorageIndex,*/
                     content: Vec<u8>,
                     _add_yj: bool,) -> Result<()>{
        println!("in update_corpus, add input content = {:?}", content);
        let hash = self.hash(&content);
        let input_corpus = self.in_corpus_dir.to_str().expect("Get input corpus dir");
        let new_add_dir = format!("{}/new_added/", input_corpus);
        let new_add_path = Path::new(&new_add_dir);

        // let _old = corpus.insert((new_add_path.to_path_buf(), idx), hash.clone());
        self.add_to_output_corpus(new_add_path, hash.clone(), content.clone())?;
        Ok(())
    }

    pub fn add_to_output_corpus(&self, new_add_path: &Path, name: String, content: Vec<u8>)
        -> Result<()> {
        let folder = new_add_path;

        if !folder.is_dir() {
            println!("if it's dir directory, create one");
            std::fs::create_dir_all(&folder)?;
        }
        let extension = "json";
        let path = folder.join(name).with_extension(extension);
        fs::write(path, content)?;
        println!("WRITE to output_corpus");
        Ok(())
    }

    /// Vec<u8> : data in byte format
    pub fn read_input_corpus(&mut self) -> Result<Vec<Vec<u8>>> {
        let mut values = vec![];
        self.read_input_corpus_rec(self.in_corpus_dir.as_path(), &mut values)?;
        Ok(values)
    }

    fn read_input_corpus_rec(&self, in_corpus: &Path, values: &mut Vec<Vec<u8>>) -> Result<()> {
        if !in_corpus.exists() {
            return Ok(());
        }
        if !in_corpus.is_dir() {
            return Result::Err(io::Error::new(
                io::ErrorKind::Other,
                "The corpus path is not a directory.",
            ));
        }
        for entry in fs::read_dir(in_corpus)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                self.read_input_corpus_rec(&path, values)?;
            } else {
                // if it is a file.
                let data = fs::read(path)?;
                values.push(data);
            }
        }
        Ok(())
    }

    /*

        fn get_input_and_subvalue_provider<'a>(
            pool_storage: &'a mut RcSlab<FuzzedInputAndSubValueProvider<T, M>>,
            sensor_and_pool: &mut dyn SensorAndPool,
            rng: &fastrand::Rng,
            idx: PoolStorageIndex,
        ) -> (&'a mut FuzzedInput<T, M>, &'a (impl SubValueProvider + 'a)) {
            let idx_cross = sensor_and_pool.get_random_index().unwrap();

            if idx == idx_cross || rng.u8(..5) == 0 {
                let FuzzedInputAndSubValueProvider { input, subvalues } = &mut pool_storage[idx.0];
                (input, subvalues)
            } else {
                // crossover of two different test cases
                let (input, FuzzedInputAndSubValueProvider { subvalues, .. }) =
                    pool_storage.get_mut_and_ref(idx.0, idx_cross.0).unwrap();
                (&mut input.input, subvalues)
            }
        }

        /// check 2
        fn process_next_input(&mut self, v: &mut Vec<Vec<usize>>) -> Result<(), ReasonForStopping<T>> {
            let FuzzerState {
                pool_storage,
                sensor_and_pool,
                input_idx,
                mutator,
                settings,
                rng,
                fuzzer_stats,
                world,
                serializer, // yunji
                ..
            } = &mut self.state;
            // let mut v : Vec<Vec<usize>> = Vec::new();

            /// TODO: get random --> get not randomly
            if let Some(idx) = sensor_and_pool.get_random_index() {
                *input_idx = FuzzerInputIndex::Pool(idx);
                let (input, subvalue_provider) =
                    Self::get_input_and_subvalue_provider(pool_storage, sensor_and_pool.as_mut(), rng, idx);
                // let tmp = serializer.to_data(&input.value.clone()); // YUNJI
                let generation = input.generation;
                if let Some((unmutate_token, complexity)) =
                    input.mutate(mutator, subvalue_provider, settings.max_input_cplx) // after mutate
                {
                    //drop(subvalue_provider);
                    /// always this branch is executed.
                    let tmp = serializer.to_data(&input.value.clone()); // YUNJI
                    let s = match String::from_utf8(tmp) {
                        Ok(vv) => vv,
                        Err(e) => panic!("error = {:?}", e),
                    };
                    println!("==Next Input = {:?} ==========", s);
                    if complexity < self.state.settings.max_input_cplx {
                        self.test_and_process_input(complexity, v)?;
                        // yunji: write path file
                    }

                    // read latest path file
                    // decide the byte is same? or not
                    // if compare byte and it's same then discard,
                    // esle add that one

                    // Retrieving the input may fail because the input may have been deleted
                    if let Some(input) = self.state.pool_storage.get_mut(idx.0).map(
                        #[no_coverage]
                            |x| &mut x.input,
                    ) && input.generation == generation {
                        input.unmutate(&self.state.mutator, unmutate_token);
                    }

                    Ok(())
                } else {
                    world.report_event(FuzzerEvent::End, Some((fuzzer_stats, sensor_and_pool.stats().as_ref())));
                    Err(ReasonForStopping::ExhaustedAllPossibleMutations)
                }
            } else if let Some((input, cplx)) = self.state.arbitrary_input() {
                println!("exeue 2");
                self.state.input_idx = FuzzerInputIndex::Temporary(input);

                if cplx < self.state.settings.max_input_cplx {
                    self.test_and_process_input(cplx, v)?;
                }

                Ok(())
            } else {
                println!("exeue 3");
                self.state.world.report_event(
                    FuzzerEvent::End,
                    Some((&self.state.fuzzer_stats, self.state.sensor_and_pool.stats().as_ref())),
                );
                Err(ReasonForStopping::ExhaustedAllPossibleMutations)
            }
        }


     */

 /*
    // 한번 반복 셋, 반복 최소단위
    /// check 1
    fn process_initial_inputs2(&mut self, v: &mut Vec<Vec<usize>>)
        -> Result<(), ReasonForStopping<T>>
    // where <M as traits::Mutator<T>>::MutationStep: std::fmt::Debug, <M as traits::Mutator<T>>::Cache: std::fmt::Debug,<M as traits::Mutator<T>>::MutationStep: std::fmt::Debug,
    {
        println!("================ process_initial_inputs ================");
        // let mut inputs : Vec<Vec<usize>> = v;
        // let value = cm.from_data();
        let mut values: Vec<CompiledModule> = self
            .read_input_corpus()
            .expect("READ_INPUT_FILE_ERROR")
            .into_iter()
            .filter_map(
                |value| {
                    // print!("*  {:?}, ", value);
                    let s = match String::from_utf8(value.clone()) {
                        // let s = match String::from_utf8(value).expect("convert yj") {
                        // let s = match std::str::from_utf8(value) {
                        Ok(vv) => vv,
                        Err(e) => panic!("error = {:?}", e),
                    };
                    println!("*  {:?}", s);
                    let cm:CompiledModule = Default::default();
                    let value = cm.from_data(&value)?;
                    println!("when is this printed? {:?}", self.state.serializer.to_data(&value));
                    // let cache = self.state.mutator.validate_value(&value)?;
                    // let mutation_step = self.state.mutator.default_mutation_step(&value, &cache);
                    // Some(FuzzedInput::new(value, cache, mutation_step, Generation(0)))
                    Some(value)
                },
            )
            .collect();

        let mut inputs = v;
        // for _ in 0..100 { // 20
        println!("================Input (2) random input ================");
        for i in 0..10 {
            if let Some((input, _)) = self.state.arbitrary_input() {
                let s = match String::from_utf8(self.state.serializer.to_data(&input.value).clone()) {
                    Ok(vv) => vv,
                    Err(e) => panic!("error = {:?}", e),
                };
                println!("*  {:?}", s);
                inputs.push(input);
            } else {
                break;
            }
        }
        for i in 0..10 {
            if let Some(value) = create_random_cm() {
                let s = match String::from_utf8(self.to_data(&value).clone()) {
                    Ok(vv) => vv,
                    Err(e) => panic!("error = {:?}", e),
                };
            }

            println!("*  {:?}", s);
            if let Some((input, _)) = self.state.arbitrary_input() {
                let s = match String::from_utf8(self.state.serializer.to_data(&input.value).clone()) {
                    Ok(vv) => vv,
                    Err(e) => panic!("error = {:?}", e),
                };
                println!("*  {:?}", s);
                inputs.push(input);
            } else {
                break;
            }
        }
        // inputs.retain(
        //     #[no_coverage]
        //         |i| i.complexity(&self.state.mutator)
        //         <= self.state.settings.max_input_cplx,
        // );
        //
        // self.state.world.set_checkpoint_instant();

        let mut cnt = 1;


        for input in inputs {
            let tmp = self.state.serializer.to_data(&input.value.clone());

            let cplx = input.complexity(&self.state.mutator);
            self.state.input_idx = FuzzerInputIndex::Temporary(input);

            let s = match String::from_utf8(tmp) {
                Ok(vv) => vv,
                Err(e) => panic!("error = {:?}", e),
            };
            println!("==== INPUT = {:?} ==============", s);
            // println!("---i guess here ------------{:?}", input.value);
            // let tmp = input.to_bytes();
            // file.write_all(tmp);
            self.test_and_process_input(cplx, v)?;

            // yunji

            // let old_name = "/home/y23kim/rust/test_progs/corpus/sub_dir/new_path";

            let arguments = std::env::var("FUZZCHECK_ARGS").unwrap();
            let args: Vec<&str> = arguments.split_whitespace().collect();
            println!("in initial stack args = {:?} {:?}", arguments,args);
            // let in_corpus = PathBuf::from(args[1]);
            // let corpus= in_corpus.as_path();

            let old_name = args[1];

            // let new_name = "/home/y23kim/rust/test_progs/corpus/sub_dir/path" + cnt;
            let new_name = format!("{}_init_{}", old_name, cnt);
            // let new_name = format!("/home/y23kim/rust/test_progs/corpus/sub_dir/path{}", cnt);
            let _ = fs::rename(old_name, new_name);
            println!("RENAME file => path{:?}", cnt);
            cnt += 1;

        }
        // println!("inp length after execution iteration = {:?}, {:?}", inputs.clone().len(), v);
        println!("==== END of initial process = {:?}", v);


        Ok(())
    }




    pub fn main_loop(&mut self) -> Result<!, ReasonForStopping<T>> {
        /// yunji value vector
        let mut v : Vec<Vec<usize>> = Vec::new();

        // self.state.world.report_event(
        //     FuzzerEvent::Start,
        //     Some((&self.state.fuzzer_stats, self.state.sensor_and_pool.stats().as_ref())),
        // );

        /// check 1
        self.process_initial_inputs(&mut v)?;
        println!("========= After initial inputs = {:?}", v);


        // self.state.world.report_event(
        //     // yj print "finished reading corpus"
        //     FuzzerEvent::DidReadCorpus,
        //     Some((&self.state.fuzzer_stats, self.state.sensor_and_pool.stats().as_ref())),
        // );

        self.state.world.set_checkpoint_instant();
        let mut next_milestone = (self.state.fuzzer_stats.total_number_of_runs + 10) * 2;
        println!("initial next milestone :{:?}, maximum_duration {:?}", next_milestone, self.state.settings.maximum_duration);
        let mut cnt = 1;


        let file_to_read = self.exec_path_file;
        loop {
            println!("in main loop");
            let duration_since_beginning = self.state.world.elapsed_time_since_start();
            if duration_since_beginning > self.state.settings.maximum_duration {
                return Err(ReasonForStopping::MaxDurationReached);
            }
            println!("now duration :{:?}",duration_since_beginning);

            // stop for testing...
            if self.state.fuzzer_stats.total_number_of_runs >= 300 {
                // if self.state.fuzzer_stats.total_number_of_runs >= self.state.settings.maximum_iterations {
                println!("*** BREAK for testing ({:?} {:?}) {:?}", cnt, self.state.fuzzer_stats.total_number_of_runs.clone(), next_milestone);
                return Err(ReasonForStopping::MaxIterationsReached);
            }

            /// yj check 2
            // test and process input is called in this function.
            self.process_next_input(&mut v)?;

            /// handle files in execution path
            let new_name = format!("{:?}_next_{}", file_to_read.to_str(), cnt);
            fs::rename(file_to_read.to_str(), new_name);
            cnt += 1;

            println!("* END of next input ({:?} {:?}) {:?}", cnt, self.state.fuzzer_stats.total_number_of_runs.clone(), next_milestone);

            // read this file
            // decide if this file is unique

            if self.state.fuzzer_stats.total_number_of_runs >= next_milestone {
                // yj check 2
                // nopt important
                println!("total_number_of_runs >= next_milestone");
                self.state.world.report_event(
                    FuzzerEvent::Pulse,
                    Some((&self.state.fuzzer_stats, self.state.sensor_and_pool.stats().as_ref())),
                );
                next_milestone = self.state.fuzzer_stats.total_number_of_runs * 2;
            }
        }
        println!("after loop = {:?}", v);

    }


  */
}
