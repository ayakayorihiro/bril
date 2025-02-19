use thiserror::Error;

// Having the #[error(...)] for all variants derives the Display trait as well
#[derive(Error, Debug)]
pub enum InterpError {
  #[error("Some memory locations have not been freed by the end of execution")]
  MemLeak,
  #[error("Trying to load from uninitialized memory")]
  UsingUninitializedMemory,
  #[error("phi node executed with no last label")]
  NoLastLabel,
  #[error("no main function defined, doing nothing")]
  NoMainFunction,
  #[error("pi node has unequal numbers of labels and args")]
  UnequalPhiNode,
  #[error("multiple functions of the same name found")]
  DuplicateFunction,
  #[error("Expected empty return for `{0}`, found value")]
  NonEmptyRetForFunc(String),
  #[error("cannot allocate `{0}` entries")]
  CannotAllocSize(i64),
  #[error("Tried to free illegal memory location base: `{0}`, offset: `{1}`. Offset must be 0.")]
  IllegalFree(usize, i64), // (base, offset)
  #[error("Uninitialized heap location `{0}` and/or illegal offset `{1}`")]
  InvalidMemoryAccess(usize, i64), // (base, offset)
  #[error("Expected `{0}` function arguments, found `{1}`")]
  BadNumFuncArgs(usize, usize), // (expected, actual)
  #[error("Expected `{0}` instruction arguments, found `{1}`")]
  BadNumArgs(usize, usize), // (expected, actual)
  #[error("Expected `{0}` labels, found `{1}`")]
  BadNumLabels(usize, usize), // (expected, actual)
  #[error("Expected `{0}` functions, found `{1}`")]
  BadNumFuncs(usize, usize), // (expected, actual)
  #[error("no function of name `{0}` found")]
  FuncNotFound(String),
  #[error("undefined variable `{0}`")]
  VarUndefined(String),
  #[error("Label `{0}` for phi node not found")]
  PhiMissingLabel(String),
  #[error("unspecified pointer type `{0:?}`")]
  ExpectedPointerType(bril_rs::Type), // found type
  #[error("Expected type `{0:?}` for function argument, found `{1:?}`")]
  BadFuncArgType(bril_rs::Type, String), // (expected, actual)
  #[error("Expected type `{0:?}` for assignment, found `{1:?}`")]
  BadAsmtType(bril_rs::Type, bril_rs::Type), // (expected, actual). For when the LHS type of an instruction is bad
  #[error("There has been an io error when trying to print: `{0:?}`")]
  IoError(Box<std::io::Error>),
}
