///! This module defines data structures for FDS input data, and the functions
///! for converting them to and from Fortran Namelists. This is inherently less
///! flexible than the Namelist data, which can hold any data, but allows us to
///! operate more flexibly on FDS input data.
///!
///! TODO: How do we deal with defaults? Should we leave a Maybe value? or should
///! we insert the defaults when we convert from the Namelists. Let's go with the
///! defaults, it allows us to ignore whether something is specified or not,
///! although it does mean we need to be aware of the version of FDS we are
///! considering. This decision is based on the fact that the primary purpose is
///! analysis of the values, not how it is specified. If we care about the
///! structure of the input itself, we can always fall back to the Namelist data.
use namelist::Namelist;

/// The Haskell data type representation of an FDS input script. The first items
/// (such as head and time) are single occurrence items. As any of these items
/// may or may not occur they are given maybe types. The other items may occur
/// zero, one or many times, and are therefore given a list type. There is
/// provision for storing namelists that are not understood for the purposes of
/// forward compatibility.
#[derive(Clone, Debug)]
pub struct FDSFile {
    head: Option<Head>,
    time: Option<Time>,
    dump: Option<Dump>,
    misc: Option<Misc>,
    meshes: Vec<Mesh>,
    reacs: Vec<Reac>,
    devcs: Vec<Devc>,
    matls: Vec<Matl>,
    surfs: Vec<Surf>,
    obsts: Vec<Obst>,
    holes: Vec<Hole>,
    hvacs: Vec<Hvac>,
    vents: Vec<Vent>,
    bndfs: Vec<Bndf>,
    isofs: Vec<Isof>,
    slcfs: Vec<Slcf>,
    ramps: Vec<Ramp>,
    props: Vec<Prop>,
    parts: Vec<Part>,
    trnxs: Vec<Trnx>,
    trnys: Vec<Trny>,
    trnzs: Vec<Trnz>,
    unknown_namelists: Vec<Namelist>,
}

impl Default for FDSFile {
    fn default() -> Self {
        FDSFile {
            head: None,
            time: None,
            dump: None,
            misc: None,
            meshes: vec![],
            reacs: vec![],
            devcs: vec![],
            matls: vec![],
            surfs: vec![],
            obsts: vec![],
            holes: vec![],
            hvacs: vec![],
            vents: vec![],
            bndfs: vec![],
            isofs: vec![],
            slcfs: vec![],
            ramps: vec![],
            props: vec![],
            parts: vec![],
            trnxs: vec![],
            trnys: vec![],
            trnzs: vec![],
            unknown_namelists: vec![],
        }
    }
}

#[derive(Clone, Debug)]
pub struct Head {
    chid: Option<String>,
    fyi: Option<String>,
    title: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Bndf {
    cell_centred: bool,
    fyi: Option<String>,
    part_id: Option<String>,
    PROP_ID: Option<String>,
    RECOUNT_DRIP: bool,
    QUANTITY: Option<String>,
    SPEC_ID: Option<String>,
    STATISTICS: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Bnde {
    CELL_CENTERED: bool,
    FYI: Option<String>,
    PART_ID: Option<String>,
    PROP_ID: Option<String>,
    QUANTITY: Option<String>,
    SPEC_ID: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Clip {
    FYI: Option<String>,
    MAXIMUM_DENSITY: f64,
    MAXIMUM_MASS_FRACTION: Vec<Vec<f64>>,
    MAXIMUM_TEMPERATURE: f64,
    MINIMUM_DENSITY: f64,
    MINIMUM_MASS_FRACTION: Vec<Vec<f64>>,
    MINIMUM_TEMPERATURE: f64,
}

#[derive(Clone, Debug)]
pub struct Ctrl {
    CONSTANT: f64,
    // , CYCLES : String
                   // , CYCLE_TIME : String

    DELAY: f64,
    DIFFERENTIAL_GAIN: f64,
    EVACUATION: bool,
    FUNCTION_TYPE: String,
    ID: String,
    INITIAL_STATE: bool,
    INTEGRAL_GAIN: f64,
    INPUT_ID: Vec<String>,
    LATCH: bool,
    N: i64,
    ON_BOUND: String,
    PROPORTIONAL_GAIN: f64,
    RAMP_ID: String,
    SETPOINT: f64,
    TARGET_VALUE: f64,
    TRIP_DIRECTION: i64,
}

#[derive(Clone, Debug)]
pub struct Csvf {
    CSVFILE: String,
    UVWFILE: String,
}

#[derive(Clone, Debug)]
pub struct Devc {
    BYPASS_FLOWRATE: f64,
    CONVERSION_ADDEND: f64,
    CONVERSION_FACTOR: f64,
    COORD_FACTOR: f64,
    CTRL_ID: Option<String>,
    DELAY: f64,
    DEPTH: f64,
    DEVC_ID: Option<String>,
    DRY: bool,
    DUCT_ID: Option<String>,
    EVACUATION: bool,
    FLOWRATE: f64,
    FYI: Option<String>,
    HIDE_COORDINATES: bool,
    ID: Option<String>,
    INITIAL_STATE: bool,
    INIT_ID: Option<String>,
    IOR: Option<i64>,
    LATCH: bool,
    MATL_ID: Option<String>,
    NODE_ID: Vec<String>,
    NO_UPDATE_DEVC_ID: Option<String>,
    NO_UPDATE_CTRL_ID: Option<String>,
    ORIENTATION: XYZ,
    ORIENTATION_NUMBER: i64,
    OUTPUT: bool,
    PART_ID: Option<String>,
    PIPE_INDEX: i64,
    POINTS: i64,
    PROP_ID: Option<String>,
    QUANTITY: Option<String>,
    QUANTITY2: Option<String>,
    QUANTITY_RANGE: (f64, f64),
    R_ID: Option<String>,
    REAC_ID: Option<String>,
    RELATIVE: bool,
    ROTATION: f64,
    SETPOINT: Option<f64>,
    SMOOTHING_FACTOR: f64,
    SPEC_ID: Option<String>,
    STATISTICS: Option<String>,
    STATISTICS_START: f64,
    SURF_ID: Option<String>,
    TIME_AVERAGED: bool,
    TIME_HISTORY: bool,
    TRIP_DIRECTION: i64,
    UNITS: Option<String>,
    VELO_INDEX: i64,
    XB: Option<XB>,
    XYZ: Option<XYZ>,
    X_ID: Option<String>,
    Y_ID: Option<String>,
    Z_ID: Option<String>,
    XYZ_UNITS: String,
}

#[derive(Clone, Debug)]
pub struct Dump {
    CLIP_RESTART_FILES: bool,
    COLUMN_DUMP_LIMIT: bool,
    CTRL_COLUMN_LIMIT: i64,
    DEVC_COLUMN_LIMIT: i64,
    DT_BNDE: f64,
    DT_BNDF: f64,
    DT_CPU: f64,
    DT_CTRL: f64,
    DT_DEVC: f64,
    DT_DEVC_LINE: f64,
    DT_FLUSH: f64,
    DT_GEOM: f64,
    DT_HRR: f64,
    DT_ISOF: f64,
    DT_MASS: f64,
    DT_PART: f64,
    DT_PL3D: f64,
    DT_PROF: f64,
    DT_RESTART: f64,
    DT_SL3D: f64,
    DT_SLCF: f64,
    EB_PART_FILE: bool,
    FLUSH_FILE_BUFFERS: bool,
    GEOM_DIAG: bool,
    MASS_FILE: bool,
    MAXIMUM_PARTICLES: i64,
    MMS_TIMER: f64,
    NFRAMES: i64,
    PLOT3D_PART_ID: Vec<String>,
    PLOT3D_QUANTITY: Vec<String>,
    PLOT3D_SPEC_ID: Vec<String>,
    PLOT3D_VELO_INDEX: Vec<i64>,
    RENDER_FILE: String,
    SIG_FIGS: i64,
    SIG_FIGS_EXP: i64,
    SMOKE3D: bool,
    SMOKE3D_QUANTITY: String,
    SMOKE3D_SPEC_ID: String,
    STATUS_FILES: bool,
    SUPPRESS_DIAGNOSTICS: bool,
    UVW_TIMER: Vec<f64>,
    VELOCITY_ERROR_FILE: bool,
    WRITE_XYZ: bool,
}

#[derive(Clone, Debug)]
pub struct Hole {
    COLOR: String,
    CTRL_ID: String,
    DEVC_ID: String,
    EVACUATION: bool,
    FYI: Option<String>,
    ID: String,
    MESH_ID: String,
    MULT_ID: String,
    RGB: RGB,
    TRANSPARENCY: f64,
    XB: XB,
}

#[derive(Clone, Debug)]
pub struct Hvac {
    AIRCOIL_ID: Option<String>,
    AMBIENT: bool,
    AREA: f64,
    CLEAN_LOSS: f64,
    COOLANT_SPECIFIC_HEAT: f64,
    COOLANT_MASS_FLOW: f64,
    COOLANT_TEMPERATURE: f64,
    CTRL_ID: String,
    DAMPER: bool,
    DEVC_ID: String,
    DIAMETER: f64,
    DUCT_ID: Vec<String>,
    DUCT_INTERP_TYPE: String,
    EFFICIENCY: Vec<f64>,
    FAN_ID: String,
    FILTER_ID: String,
    FIXED_Q: Vec<f64>,
    ID: String,
    LEAK_ENTHALPY: bool,
    LENGTH: f64,
    LOADING: Vec<f64>,
    LOADING_MULTIPLIER: Vec<f64>,
    LOSS: Vec<f64>,
    MASS_FLOW: f64,
    MAX_FLOW: f64,
    MAX_PRESSURE: f64,
    N_CELLS: i64,
    NODE_ID: Vec<String>,
    PERIMETER: f64,
    RAMP_ID: String,
    RAMP_LOSS: String,
    REVERSE: bool,
    ROUGHNESS: f64,
    SPEC_ID: String,
    TAU_AC: f64,
    TAU_FAN: f64,
    TAU_VF: f64,
    TYPE_ID: String,
    VENT_ID: Option<String>,
    VENT2_ID: Option<String>,
    VOLUME_FLOW: f64,
    XYZ: XYZ,
}

#[derive(Clone, Debug)]
pub struct Init {
    AUTO_IGNITION_TEMPERATURE: f64,
    CELL_CENTERED: bool,
    CTRL_ID: String,
    DENSITY: f64,
    DEVC_ID: String,
    DIAMETER: f64,
    DT_INSERT: f64,
    DX: f64,
    DY: f64,
    DZ: f64,
    HEIGHT: f64,
    HRRPUV: f64,
    ID: String,
    MASS_FRACTION: Vec<f64>,
    MASS_PER_TIME: f64,
    MASS_PER_VOLUME: f64,
    MULT_ID: String,
    N_PARTICLES: i64,
    N_PARTICLES_PER_CELL: i64,
    PART_ID: String,
    RADIUS: f64,
    SHAPE: String,
    SPEC_ID: Vec<String>,
    TEMPERATURE: f64,
    UVW: Vec<f64>,
    XB: XB,
    XYZ: XYZ,
    PARTICLE_WEIGHT_FACTOR: f64,
    NUMBER_INITIAL_PARTICLES: i64,
}

#[derive(Clone, Debug)]
pub struct Isof {
    FYI: Option<String>,
    QUANTITY: String,
    SPEC_ID: String,
    VALUE: f64,
    VELO_INDEX: i64,
}

#[derive(Clone, Debug)]
pub struct Matl {
    A: Vec<f64>,
    ABSORPTION_COEFFICIENT: f64,
    BOILING_TEMPERATURE: f64,
    COLOR: String,
    CONDUCTIVITY: f64,
    CONDUCTIVITY_RAMP: String,
    DENSITY: f64,
    E: Vec<f64>,
    EMISSIVITY: f64,
    FYI: Option<String>,
    HEATING_RATE: Vec<f64>,
    HEAT_OF_COMBUSTION: Vec<f64>,
    HEAT_OF_REACTION: Vec<f64>,
    ID: String,
    MATL_ID: Vec<String>,
    NU_MATL: Vec<f64>,
    NU_SPEC: Vec<f64>,
    N_REACTIONS: i64,
    N_S: Vec<f64>,
    N_T: Vec<f64>,
    N_O2: Vec<f64>,
    PCR: Vec<bool>,
    PYROLYSIS_RANGE: Vec<f64>,
    REFERENCE_RATE: Vec<f64>,
    REFERENCE_TEMPERATURE: Vec<f64>,
    RGB: RGB,
    SPECIFIC_HEAT: f64,
    SPECIFIC_HEAT_RAMP: String,
    SPEC_ID: Vec<String>,
    THRESHOLD_SIGN: Vec<f64>,
    THRESHOLD_TEMPERATURE: Vec<f64>,
    // , POROSITY : String
    ALLOW_SHRINKING: bool,
    ALLOW_SWELLING: bool,
    GAS_DIFFUSION_DEPTH: Vec<f64>,
}

// data PyrolysisReac
//     = PyrolysisReacAE
//         { pyrolysisReacAE_A : f64 -- ^A
//         , pyrolysisReacAE_E : f64 -- ^E
//         , pyrolysisReacAE_HEAT_OF_REACTION : f64 -- ^HEAT_OF_REACTION
//         }
//     | PyrolysisReacTGM
//         { pyrolysisReacTGM_REFERENCE_TEMPERATURE : f64 -- ^REFERENCE_TEMPERATURE
//         , pyrolysisReacTGM_HEATING_RATE : f64 -- ^HEATING_RATE
//         , pyrolysisReacTGM_PYROLYSIS_RANGE : f64 -- ^PYROLYSIS_RANGE
//         , pyrolysisReacTGM_HEAT_OF_REACTION : f64 -- ^HEAT_OF_REACTION
//         }
//     | NoPyrolysis

// ----------------------------------------
#[derive(Clone, Debug)]
pub struct Mesh {
    ID: Option<String>,
    XB: XB,
    IJK: IJK,
    COLOR: String,
    CYLINDRICAL: bool,
    EVACUATION: bool,
    EVAC_HUMANS: bool,
    EVAC_Z_OFFSET: f64,
    FYI: Option<String>,
    LEVEL: i64,
    MPI_PROCESS: Option<i64>,
    MULT_ID: Option<String>,
    RGB: RGB,
    N_THREADS: Option<i64>, // , PERIODIC_MESH_IDS : [Text]
}

#[derive(Clone, Debug)]
pub struct Misc {
    AGGLOMERATION: bool,
    AEROSOL_AL2O3: bool,
    ALLOW_SURFACE_PARTICLES: bool,
    ALLOW_UNDERSIDE_PARTICLES: bool,
    ASSUMED_GAS_TEMPERATURE: f64,
    ASSUMED_GAS_TEMPERATURE_RAMP: String,
    BAROCLINIC: bool,
    BNDF_DEFAULT: bool,
    CC_IBM: bool,
    CNF_CUTOFF: f64,
    CFL_MAX: f64,
    CFL_MIN: f64,
    CFL_VELOCITY_NORM: i64,
    CHECK_HT: bool,
    CHECK_REALIZABILITY: bool,
    CHECK_VN: bool,
    CLIP_MASS_FRACTION: bool,
    COMPUTE_VISCOSITY_TWICE: bool,
    COMPUTE_ZETA_SOURCE_TERM: bool,
    CONSTANT_H_SOLID: bool,
    CONSTANT_SPECIFIC_HEAT_RATIO: bool,
    CORIOLIS_VECTOR: Vec<f64>,
    CORRECT_SUBGRID_TEMPERATURE: bool,
    COUPLED_1D3D_HEAT_TRANSFER: bool,
    C_DEARDORFF: f64,
    C_RNG: f64,
    C_RNG_CUTOFF: f64,
    C_SMAGORINSKY: f64,
    C_VREMAN: f64,
    DNS: bool,
    DRAG_CFL_MAX: f64,
    DT_MEAN_FORCING: f64,
    ENTHALPY_TRANSPORT: bool,
    EVACUATION_DRILL: bool,
    EVACUATION_MC_MODE: bool,
    EVAC_PRESSURE_ITERATIONS: i64,
    EVAC_SURF_DEFAULT: String,
    EVAC_TIME_ITERATIONS: i64,
    EVAPORATION: bool,
    // , EXCHANGE_EDGES : String
    EXTERNAL_BOUNDARY_CORRECTION: bool,
    EXTINCTION_MODEL: String,
    HVAC_PRES_RELAX: f64,
    HT3D_TEST: i64,
    FDS5_OPTIONS: bool,
    FLUX_LIMITER: i64,
    FORCE_VECTOR: Vec<f64>,
    FREEZE_VELOCITY: bool,
    FYI: Option<String>,
    GAMMA: f64,
    GRAVITATIONAL_DEPOSITION: bool,
    GRAVITATIONAL_SETTLING: bool,
    GROUND_LEVEL: f64,
    GVEC: Vec<f64>,
    DT_HVAC: f64,
    H_F_REFERENCE_TEMPERATURE: f64,
    HRRPUV_MAX_SMV: f64,
    HUMIDITY: f64,
    HVAC_MASS_TRANSPORT: bool,
    IBLANK_SMV: bool,
    IMMERSED_BOUNDARY_METHOD: i64,
    INITIAL_UNMIXED_FRACTION: f64, // , KINETIC_ENERGY_SOURCE : String
    LAPSE_RATE: f64,
    LES_FILTER_WIDTH: String,
    MAX_CHEMISTRY_ITERATIONS: i64,
    MAX_LEAK_PATHS: i64,
    MAXIMUM_VISIBILITY: f64,
    MEAN_FORCING: Vec<bool>,
    MPI_TIMEOUT: f64,
    N_FIXED_CHEMISTRY_SUBSTEPS: i64,
    NEAR_WALL_TURBULENCE_MODEL: String, // , NEW_MOMENTUM_NUDGING : String
                                        // , NEW_OPEN_BOUNDARY : String
    NOISE: bool,
    NOISE_VELOCITY: f64,
    NO_EVACUATION: bool,
    NO_RAMPS: bool, // , NORTHANGLE : String
    OVERWRITE: bool,
    PARTICLE_CFL_MAX: f64,
    PARTICLE_CFL_MIN: f64,
    PARTICLE_CFL: bool,
    PERIODIC_TEST: i64, // , PROFILING : String
    POROUS_FLOOR: bool, // , POTENTIAL_TEMPERATURE_CORRECTION : String
    PR: f64,
    PROCESS_ALL_MESHES: bool,
    PROJECTION: bool,
    P_INF: f64, // , RAMP_FVX_T : String
                // , RAMP_FVY_T : String
                // , RAMP_FVZ_T : String
    RAMP_GX: String,
    RAMP_GY: String,
    RAMP_GZ: String,
    RAMP_U0: String,
    RAMP_U0_T: String,
    RAMP_V0: String,
    RAMP_V0_T: String,
    RAMP_W0: String,
    RAMP_W0_T: String,
    RAMP_U0_Z: String,
    RAMP_V0_Z: String,
    RAMP_W0_Z: String, // , RADIATION : String
    RESEARCH_MODE: bool,
    RESTART: bool,
    RESTART_CHID: String,
    RICHARDSON_ERROR_TOLERANCE: f64,
    RUN_AVG_FAC: f64,
    SC: f64,
    SECOND_ORDER_INTERPOLATED_BOUNDARY: bool,
    SECOND_ORDER_PARTICLE_TRANSPORT: bool,
    SHARED_FILE_SYSTEM: bool, // , SLIP_CONDITION : String
    SMOKE_ALBEDO: f64,
    SOLID_PHASE_ONLY: bool, // , SOOT_OXIDATION : String
                            // , SPONGE_LAYER_DISTANCE : String
    STRATIFICATION: bool,
    SUPPRESSION: bool, // , SURF_DEFAULT : String
                       // , TEMPERATURE_DEPENDENT_REACTION : String
                       // , TENSOR_DIFFUSIVITY : String
    TERRAIN_CASE: bool,
    TERRAIN_IMAGE: String, // , TEST_FILTER_QUADRATURE : String
    TEXTURE_ORIGIN: Vec<f64>,
    THERMOPHORETIC_DEPOSITION: bool,
    THICKEN_OBSTRUCTIONS: bool, // , TRANSPORT_UNMIXED_FRACTION : String
                                // , TRANSPORT_ZETA_SCHEME : String
    TMPA: f64,
    TURBULENCE_MODEL: String,
    TURBULENT_DEPOSITION: bool, // , TURB_INIT_CLOCK : String
    U0: f64,
    UVW_FILE: String,
    V0: f64,
    VEG_LEVEL_SET_COUPLED: bool,
    VEG_LEVEL_SET_UNCOUPLED: bool,
    VERBOSE: f64,
    VISIBILITY_FACTOR: f64,
    VN_MAX: f64,
    VN_MIN: f64,
    Y_CO2_INFTY: f64,
    Y_O2_INFTY: f64,
    W0: f64, // , WD_PROPS : String
             // , WIND_BOUNDARY : String
             // , WIND_ONLY : String
}

#[derive(Clone, Debug)]
pub struct Mult {
    DX: f64,
    DXB: Vec<f64>,
    DX0: f64,
    DY: f64,
    DYB: Vec<f64>,
    DY0: f64,
    DZ: f64,
    DZB: Vec<f64>,
    DZ0: f64,
    ID: String,
    I_LOWER: i64,
    I_UPPER: i64,
    J_LOWER: i64,
    J_UPPER: i64,
    K_LOWER: i64,
    K_UPPER: i64,
    N_LOWER: i64,
    N_UPPER: i64,
}

#[derive(Clone, Debug)]
pub struct Obst {
    ALLOW_VENT: bool,
    BNDF_FACE: (bool, bool, bool, bool, bool, bool),
    BNDF_OBST: bool,
    BULK_DENSITY: Option<f64>,
    COLOR: Option<String>,
    CTRL_ID: Option<String>,
    DEVC_ID: Option<String>,
    EVACUATION: bool,
    FYI: Option<String>,
    HT3D: bool,
    ID: Option<String>,
    MATL_ID: Option<String>,
    MESH_ID: Option<String>,
    MULT_ID: Option<String>,
    // , NOTERRAIN : bool
    OUTLINE: bool,
    OVERLAY: bool,
    PERMIT_HOLE: bool,
    PROP_ID: Option<String>,
    REMOVABLE: bool,
    RGB: Option<RGB>,
    SURF_ID: Option<String>,
    SURF_ID6: Option<(String, String, String, String, String, String)>,
    SURF_IDS: Option<(String, String, String)>,
    TEXTURE_ORIGIN: XYZ,
    THICKEN: bool,
    TRANSPARENCY: f64,
    XB: XB,
}

#[derive(Clone, Debug)]
pub struct Part {
    AGE: f64,
    BREAKUP: bool,
    BREAKUP_CNF_RAMP_ID: Option<String>,
    BREAKUP_DISTRIBUTION: String,
    BREAKUP_GAMMA_D: f64,
    BREAKUP_RATIO: f64,
    BREAKUP_SIGMA_D: Option<f64>,
    CHECK_DISTRIBUTION: bool,
    CNF_RAMP_ID: Option<String>,
    COLOR: String,
    COMPLEX_REFRACTIVE_INDEX: f64,
    CTRL_ID: Option<String>,
    DENSE_VOLUME_FRACTION: f64,
    DEVC_ID: Option<String>,
    DIAMETER: Option<f64>,
    DISTRIBUTION: String,
    DRAG_COEFFICIENT: Vec<f64>,
    DRAG_LAW: String,
    FREE_AREA_FRACTION: Option<f64>,
    FYI: Option<String>,
    GAMMA_D: f64,
    HEAT_OF_COMBUSTION: Option<f64>,
    HORIZONTAL_VELOCITY: f64,
    ID: Option<String>,
    INITIAL_TEMPERATURE: f64,
    MASSLESS: bool,
    MAXIMUM_DIAMETER: f64,
    MINIMUM_DIAMETER: f64,
    MONODISPERSE: bool,
    N_STRATA: i64,
    ORIENTATION: Vec<f64>,
    PERMEABILITY: Vec<f64>,
    PERIODIC_X: bool,
    PERIODIC_Y: bool,
    PERIODIC_Z: bool,
    POROUS_VOLUME_FRACTION: Option<f64>,
    PROP_ID: Option<String>,
    QUANTITIES: Vec<String>,
    QUANTITIES_SPEC_ID: Vec<String>,
    RADIATIVE_PROPERTY_TABLE: Option<f64>,
    REAL_REFRACTIVE_INDEX: f64,
    RGB: Option<RGB>,
    RUNNING_AVERAGE_FACTOR: f64,
    SAMPLING_FACTOR: i64,
    SECOND_ORDER_PARTICLE_TRANSPORT: bool,
    SIGMA_D: Option<f64>,
    SPEC_ID: Option<String>,
    STATIC: bool,
    SURFACE_TENSION: f64,
    SURF_ID: Option<String>,
    TARGET_ONLY: bool,
    TURBULENT_DISPERSION: bool,
    VERTICAL_VELOCITY: f64,
}

#[derive(Clone, Debug)]
pub struct Pres {
    CHECK_POISSON: bool,
    FISHPAK_BC: Vec<i64>,
    // , GLMAT_SOLVER : String
    ITERATION_SUSPEND_FACTOR: f64,
    // , LAPLACE_PRESSURE_CORRECTION : String
    MAX_PRESSURE_ITERATIONS: i64,
    PRESSURE_RELAX_TIME: f64,
    PRESSURE_TOLERANCE: f64,
    RELAXATION_FACTOR: f64,
    SCARC_METHOD: String,
    SCARC_KRYLOV: String,
    SCARC_MULTIGRID: String,
    SCARC_SMOOTH: String,
    SCARC_PRECON: String,
    SCARC_COARSE: String,
    SCARC_INITIAL: String,
    SCARC_ACCURACY: f64,
    SCARC_DEBUG: String,
    SCARC_MULTIGRID_CYCLE: String,
    SCARC_MULTIGRID_LEVEL: String,
    SCARC_MULTIGRID_COARSENING: String,
    SCARC_MULTIGRID_ITERATIONS: i64,
    SCARC_MULTIGRID_ACCURACY: f64,
    SCARC_MULTIGRID_INTERPOL: String,
    SCARC_KRYLOV_ITERATIONS: i64,
    SCARC_KRYLOV_ACCURACY: f64,
    SCARC_SMOOTH_ITERATIONS: i64,
    SCARC_SMOOTH_ACCURACY: f64,
    SCARC_SMOOTH_OMEGA: String,
    SCARC_PRECON_ITERATIONS: i64,
    SCARC_PRECON_ACCURACY: f64,
    SCARC_PRECON_OMEGA: String,
    SCARC_COARSE_ITERATIONS: i64,
    SCARC_COARSE_ACCURACY: f64,
    SOLVER: String,
    SUSPEND_PRESSURE_ITERATIONS: i64,
    VELOCITY_TOLERANCE: f64,
}

#[derive(Clone, Debug)]
pub struct Prof {
    FORMAT_INDEX: i64,
    FYI: String,
    ID: String,
    IOR: i64,
    QUANTITY: String,
    XYZ: XYZ,
}

#[derive(Clone, Debug)]
pub struct Prop {
    ACTIVATION_OBSCURATION: f64,
    ACTIVATION_TEMPERATURE: f64,
    ALPHA_C: f64,
    ALPHA_E: f64,
    BEAD_DENSITY: f64,
    BEAD_DIAMETER: f64,
    BEAD_EMISSIVITY: f64,
    BEAD_HEAT_TRANSFER_COEFFICIENT: f64,
    BEAD_SPECIFIC_HEAT: f64,
    BETA_C: f64,
    BETA_E: f64,
     // , FED_ACTIVITY : String
    CHARACTERISTIC_VELOCITY: f64,
    C_FACTOR: f64,
    DENSITY: f64,
    DIAMETER: f64,
    DROPLET_VELOCITY: f64,
    EMISSIVITY: f64,
    FLOW_RAMP: String,
    FLOW_RATE: f64,
    FLOW_TAU: f64,
    FYI: Option<String>,
    GAUGE_EMISSIVITY: f64,
    GAUGE_TEMPERATURE: f64,
    HEAT_TRANSFER_COEFFICIENT: f64,
    ID: Option<String>,
    INITIAL_TEMPERATURE: f64,
    K_FACTOR: f64,
    LENGTH: f64,
    MASS_FLOW_RATE: f64,
    OFFSET: f64,
    OPERATING_PRESSURE: f64,
    ORIFICE_DIAMETER: f64,
    P0: String,
    PARTICLES_PER_SECOND: i64,
    PARTICLE_VELOCITY: f64,
    PART_ID: String,
    PDPA_END: f64,
    PDPA_HISTOGRAM: bool,
    PDPA_HISTOGRAM_LIMITS: Vec<f64>,
    PDPA_HISTOGRAM_NBINS: i64,
    PDPA_HISTOGRAM_CUMULATIVE: bool,
    PDPA_INTEGRATE: bool,
    PDPA_M: i64,
    PDPA_N: i64,
    PDPA_NORMALIZE: bool,
    PDPA_RADIUS: f64,
    PDPA_START: f64,
    PRESSURE_RAMP: String, // , PX : String
                           // , PXX : String
    QUANTITY: Option<String>,
    RTI: f64,
    SMOKEVIEW_ID: Vec<String>,
    SMOKEVIEW_PARAMETERS: Vec<String>,
    SPEC_ID: String,
    SPRAY_ANGLE: Vec<f64>,
    SPRAY_PATTERN_BETA: f64,
    SPRAY_PATTERN_MU: f64,
    SPRAY_PATTERN_SHAPE: String,
    SPRAY_PATTERN_TABLE: String,
    VELOCITY_COMPONENT: i64, // , DROPLET_VELOCITY : String
}

#[derive(Clone, Debug)]
pub struct Radi {
    ANGLE_INCREMENT: i64,
    BAND_LIMITS: Vec<f64>,
    C_MAX: f64,
    C_MIN: f64,
    INITIAL_RADIATION_ITERATIONS: i64,
    KAPPA0: f64,
    NMIEANG: i64,
    NUMBER_RADIATION_ANGLES: i64,
    PATH_LENGTH: f64,
    RADIATION: bool,
    RADIATION_ITERATIONS: i64,
    // , RADIATIVE_FRACTION : String
    RADTMP: f64,
    // , RTE_SOURCE_CORRECTION : String
    TIME_STEP_INCREMENT: i64,
    WIDE_BAND_MODEL: bool,
    MIE_MINIMUM_DIAMETER: f64,
    MIE_MAXIMUM_DIAMETER: f64,
    MIE_NDG: i64,
    NUMBER_INITIAL_ITERATIONS: i64,
    QR_CLIP: f64,
}

#[derive(Clone, Debug)]
pub struct Ramp {
    ID: String,
    entries: Vec<RampEntry>,
}

#[derive(Clone, Debug)]
pub struct RampEntry {
    CTRL_ID: String,
    DEVC_ID: String,
    F: f64,
    FYI: Option<String>,
    NUMBER_INTERPOLATION_POINTS: i64,
    T: f64,
    X: f64,
    Z: f64,
}

#[derive(Clone, Debug)]
pub struct Reac {
    A: Option<f64>,
    // , ALT_REAC_ID : String
    AUTO_IGNITION_TEMPERATURE: f64,
    C: f64,
    CHECK_ATOM_BALANCE: bool,
    CO_YIELD: f64,
    CRITICAL_FLAME_TEMPERATURE: f64,
    E: f64,
    EPUMO2: f64,
    // , K : String
    EQUATION: String,
    FIXED_MIX_TIME: f64,
    // , FLAME_SPEED : String
    // , FLAME_SPEED_EXPONENT : String
    // , FLAME_SPEED_TEMPERATURE : String
    FORMULA: String,
    FUEL: String,
    FUEL_RADCAL_ID: String,
    // , FWD_ID : String
    FYI: Option<String>,
    H: f64,
    HEAT_OF_COMBUSTION: f64,
    ID: Option<String>,
    IDEAL: bool,
    N: f64,
    NU: Vec<f64>,
    N_S: Vec<f64>,
    N_T: f64,
    O: f64,
    // , ODE_SOLVER : String
    RADIATIVE_FRACTION: f64,
    RAMP_CHI_R: String,
    // , RAMP_FS : String
    REAC_ATOM_ERROR: f64,
    REAC_MASS_ERROR: f64,
    // , REVERSE : String
    SOOT_H_FRACTION: f64,
    SOOT_YIELD: f64,
    SPEC_ID_N_S: Vec<String>,
    SPEC_ID_NU: Vec<String>,
    // , TABLE_FS : String
    // , TAU_CHEM : String
    // , TAU_FLAME : String
    THIRD_BODY: bool,
    // , TURBULENT_FLAME_SPEED_ALPHA : String
    // , TURBULENT_FLAME_SPEED_EXPONENT : String
    // , Y_P_MIN_EDC : String
}

#[derive(Clone, Debug)]
pub struct Slcf {
    AGL_SLICE: String,
    CELL_CENTERED: bool,
    EVACUATION: bool, // , FACE_CENTERED : String
                      // , FIRE_LINE : String
    FYI: Option<String>,
    ID: Option<String>,
    IOR: i64,
    LEVEL_SET_FIRE_LINE: String,
    MAXIMUM_VALUE: f64,
    MESH_NUMBER: i64,
    MINIMUM_VALUE: f64,
    PART_ID: String,
    PBX: Option<f64>,
    PBY: Option<f64>,
    PBZ: Option<f64>, // , PROP_ID : String
    QUANTITY: Option<String>,
    QUANTITY2: Option<String>,
    REAC_ID: String, // , SLICETYPE : String
    SPEC_ID: Option<String>,
    VECTOR: bool,
    VELO_INDEX: i64,
    XB: XB,
}

#[derive(Clone, Debug)]
pub struct Spec {
    AEROSOL: bool,
    ALIAS: String,
    BACKGROUND: bool, // , COPY_LUMPED : String
    CONDUCTIVITY: f64,
    CONDUCTIVITY_SOLID: f64,
    DENSITY_LIQUID: f64,
    DENSITY_SOLID: f64,
    DIFFUSIVITY: f64,
    ENTHALPY_OF_FORMATION: f64,
    EPSILONKLJ: f64,
    FIC_CONCENTRATION: f64,
    FLD_LETHAL_DOSE: f64,
    FORMULA: String,
    FYI: Option<String>,
    HEAT_OF_VAPORIZATION: f64,
    H_V_REFERENCE_TEMPERATURE: f64,
    ID: String,
    LUMPED_COMPONENT_ONLY: bool,
    MASS_EXTINCTION_COEFFICIENT: f64,
    MASS_FRACTION: Vec<f64>,
    MASS_FRACTION_0: f64, // , MAX_DIAMETER : String
    MEAN_DIAMETER: f64,
    MELTING_TEMPERATURE: f64, // , MIN_DIAMETER : String
    MW: f64, // , N_BINS : String
    PR_GAS: f64,
    PRIMITIVE: bool,
    RADCAL_ID: String,
    RAMP_CP: String,
    RAMP_CP_L: String,
    RAMP_D: String,
    RAMP_G_F: String,
    RAMP_K: String,
    RAMP_MU: String,
    REFERENCE_ENTHALPY: f64,
    REFERENCE_TEMPERATURE: f64,
    SIGMALJ: f64,
    SPEC_ID: Vec<String>,
    SPECIFIC_HEAT: f64,
    SPECIFIC_HEAT_LIQUID: f64,
    VAPORIZATION_TEMPERATURE: f64,
    VISCOSITY: f64,
    VOLUME_FRACTION: Vec<f64>,
}

#[derive(Clone, Debug)]
pub struct Surf {
    ADIABATIC: bool,
    AUTO_IGNITION_TEMPERATURE: f64,
    BACKING: String,
    BURN_AWAY: bool,
    CELL_SIZE_FACTOR: f64,
    C_FORCED_CONSTANT: f64,
    C_FORCED_PR_EXP: f64,
    C_FORCED_RE: f64,
    C_FORCED_RE_EXP: f64,
    C_HORIZONTAL: f64,
    C_VERTICAL: f64,
    COLOR: Option<String>,
    CONVECTION_LENGTH_SCALE: f64,
    CONVECTIVE_HEAT_FLUX: f64,
    CONVERT_VOLUME_TO_MASS: bool,
    DEFAULT: bool,
    DT_INSERT: f64,
    EMISSIVITY: f64,
    EMISSIVITY_BACK: f64,
    EVAC_DEFAULT: bool,
    EXTERNAL_FLUX: f64,
    E_COEFFICIENT: f64,
    FIRELINE_MLR_MAX: f64,
    FREE_SLIP: bool,
    FYI: Option<String>,
    GEOMETRY: String,
    HEAT_OF_VAPORIZATION: f64,
    HEAT_TRANSFER_COEFFICIENT: f64,
    HEAT_TRANSFER_COEFFICIENT_BACK: f64,
    HEAT_TRANSFER_MODEL: String,
    HRRPUA: Option<f64>,
    HT3D: bool,
    ID: Option<String>,
    IGNITION_TEMPERATURE: f64,
    INNER_RADIUS: f64,
    INTERNAL_HEAT_SOURCE: Vec<f64>,
    LAYER_DIVIDE: f64,
    LEAK_PATH: Vec<i64>,
    LENGTH: f64,
    MASS_FLUX: Option<Vec<f64>>,
    MASS_FLUX_TOTAL: Option<f64>,
    MASS_FLUX_VAR: Option<f64>,
    MASS_FRACTION: Vec<f64>,
    MASS_TRANSFER_COEFFICIENT: f64,
    MATL_ID: Vec<String>,
    MATL_MASS_FRACTION: Vec<f64>,
    MINIMUM_LAYER_THICKNESS: f64,
    MLRPUA: Option<f64>, // , N_CELLS_MAX : String
    N_LAYER_CELLS_MAX: Vec<i64>,
    NET_HEAT_FLUX: f64,
    NO_SLIP: bool,
    NPPC: i64,
    PARTICLE_MASS_FLUX: f64,
    PART_ID: String,
    PLE: f64,
    PROFILE: String,
    RADIUS: f64,
    RAMP_EF: String,
    RAMP_MF: Vec<String>,
    RAMP_PART: String,
    RAMP_Q: Option<String>,
    RAMP_T: Option<String>,
    RAMP_T_I: Option<String>,
    RAMP_V: Option<String>,
    RAMP_V_X: Option<String>,
    RAMP_V_Y: Option<String>,
    RAMP_V_Z: Option<String>,
    RGB: RGB,
    ROUGHNESS: f64,
    SPEC_ID: String,
    SPREAD_RATE: f64,
    STRETCH_FACTOR: Vec<f64>,
    TAU_EF: f64,
    TAU_MF: f64,
    TAU_PART: f64,
    TAU_Q: f64,
    TAU_T: f64,
    TAU_V: f64,
    TEXTURE_HEIGHT: f64,
    TEXTURE_MAP: String,
    TEXTURE_WIDTH: f64,
    TGA_ANALYSIS: bool,
    TGA_FINAL_TEMPERATURE: f64,
    TGA_HEATING_RATE: f64,
    THICKNESS: Vec<f64>,
    TMP_BACK: f64,
    TMP_FRONT: f64,
    TMP_INNER: Vec<f64>,
    TRANSPARENCY: f64,
    VEGETATION: bool,
    VEGETATION_ARRHENIUS_DEGRAD: bool,
    VEGETATION_CDRAG: f64,
    VEGETATION_CHAR_FRACTION: f64,
    VEGETATION_ELEMENT_DENSITY: i64,
    VEGETATION_GROUND_TEMP: f64,
    VEGETATION_HEIGHT: f64,
    VEGETATION_INITIAL_TEMP: f64,
    VEGETATION_LAYERS: i64,
    VEGETATION_LINEAR_DEGRAD: bool,
    VEGETATION_LOAD: f64,
    VEGETATION_LSET_IGNITE_TIME: f64,
    VEG_LSET_QCON: f64,
    VEGETATION_MOISTURE: f64,
    VEGETATION_NO_BURN: bool,
    VEGETATION_SVRATIO: i64,
    VEG_LEVEL_SET_SPREAD: bool,
    VEG_LSET_ROS_BACK: f64,
    VEG_LSET_ROS_FLANK: f64,
    VEG_LSET_ROS_HEAD: f64,
    VEG_LSET_WIND_EXP: f64,
    VEG_LSET_SIGMA: f64,
    VEG_LSET_HT: f64,
    VEG_LSET_BETA: f64,
    VEG_LSET_ELLIPSE: f64,
    VEG_LSET_TAN2: bool,
    VEG_LSET_ELLIPSE_HEAD: f64,
    VEL: Option<f64>,
    VEL_BULK: f64,
    VEL_GRAD: f64,
    VEL_T: Option<(f64, f64)>,
    VOLUME_FLOW: Option<f64>,
    WIDTH: f64,
    XYZ: XYZ,
    Z0: f64, // , ZETA_FRONT : String
             // , EXTERNAL_FLUX_RAMP : String
             // , TAU_EXTERNAL_FLUX : String
             // , VOLUME_FLUX : String
}

// #[derive(Clone, Debug)]
// pub struct SurfLayer
//
//
//     { surfLayer_THICKNESS : f64
//     , surfLayer_Components : [SurfLayerComponent]
//     }

// #[derive(Clone, Debug)]
// pub struct SurfLayerComponent
// --    { surfLayerComponent_Pos : i64
// --    , surfLayerComponent_ParentLayer : SurfLayer
//     { surfLayerComponent_MATL : Matl
// --    { surfLayerComponent_MATL : String
//     , surfLayerComponent_MATL_MASS_FRACTION : f64
//     }

// --surfLayerComponent_Pos = surfLayerComponent_Pos
// --surfLayerComponent_ParentLayer = surfLayerComponent_ParentLayer
// -- surfLayerComponent_MATL = surfLayerComponent_MATL
// -- surfLayerComponent_MATL_MASS_FRACTION = (\a -> a) . surfLayerComponent_MATL_MASS_FRACTION
// #[derive(Clone, Debug)]
// pub struct SurfBurner
// --    { surfLayerComponent_Pos : i64
// --    , surfLayerComponent_ParentLayer : SurfLayer
// --    { surfLayerComponent_MATL : Matl
//     { surfBurner_HRRPUA : f64
//     , surfBURNER_TAU_Q : f64
//     }
//     | NoBurner

#[derive(Clone, Debug)]
pub struct Tabl {
    FYI: Option<String>,
    ID: String,
    TABLE_DATA: Vec<f64>,
}

#[derive(Clone, Debug)]
pub struct Time {
    DT: Option<f64>,
    EVAC_DT_FLOWFIELD: f64,
    EVAC_DT_STEADY_STATE: f64,
    FYI: Option<String>,
    LIMITING_DT_RATIO: f64,
    LOCK_TIME_STEP: bool,
    RESTRICT_TIME_STEP: bool,
    T_BEGIN: f64,
    T_END: f64,
    T_END_GEOM: f64,
    TIME_SHRINK_FACTOR: f64,
    WALL_INCREMENT: i64,
    WALL_INCREMENT_HT3D: i64,
    TWFIN: f64,
}

#[derive(Clone, Debug)]
pub struct Trnx {
    CC: f64,
    FYI: Option<String>,
    IDERIV: i64,
    MESH_NUMBER: i64,
    PC: f64,
}

#[derive(Clone, Debug)]
pub struct Trny {
    CC: f64,
    FYI: Option<String>,
    IDERIV: i64,
    MESH_NUMBER: i64,
    PC: f64,
}

#[derive(Clone, Debug)]
pub struct Trnz {
    CC: f64,
    FYI: Option<String>,
    IDERIV: i64,
    MESH_NUMBER: i64,
    PC: f64,
}

#[derive(Clone, Debug)]
pub struct Vent {
    COLOR: Option<String>,
    CTRL_ID: String,
    DEVC_ID: String,
    DYNAMIC_PRESSURE: f64,
    EVACUATION: bool,
    FYI: Option<String>,
    ID: Option<String>,
    IOR: i64,
    L_EDDY: f64,
    L_EDDY_IJ: Vec<i64>,
    MB: String,
    MESH_ID: String,
    MULT_ID: String,
    N_EDDY: i64,
    OUTLINE: bool,
    PBX: f64,
    PBY: f64,
    PBZ: f64,
    PRESSURE_RAMP: String,
    RADIUS: f64,
    REYNOLDS_STRESS: Vec<f64>,
    RGB: RGB,
    SPREAD_RATE: f64,
    SURF_ID: Option<String>,
    TEXTURE_ORIGIN: Vec<f64>,
    TMP_EXTERIOR: f64,
    TMP_EXTERIOR_RAMP: String,
    TRANSPARENCY: f64,
    UVW: Vec<f64>,
    VEL_RMS: f64,
    // , WIND : String
    XB: Option<XB>,
    XYZ: XYZ,
}

#[derive(Clone, Debug)]
pub struct Zone {
    id: String,
    LEAK_AREA: f64,
    LEAK_PRESSURE_EXPONENT: f64,
    LEAK_REFERENCE_PRESSURE: f64,
    XB: XB,
    PERIODIC: bool,
}

#[derive(Clone, Debug)]
pub enum Plane {
    X,
    Y,
    Z,
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    NegX,
    PosX,
    NegY,
    PosY,
    NegZ,
    PosZ,
}

#[derive(Clone, Debug)]
pub struct IJK {
    ijk_i: i64,
    ijk_j: i64,
    ijk_k: i64,
}

#[derive(Clone, Debug)]
pub struct RGB {
    rgb_r: i64,
    rgb_g: i64,
    rgb_b: i64,
}

#[derive(Clone, Debug)]
pub struct XB {
    x1: Coord,
    x2: Coord,
    y1: Coord,
    y2: Coord,
    z1: Coord,
    z2: Coord,
}

#[derive(Clone, Debug)]
pub struct XYZ {
    xyz_x: Coord,
    xyz_y: Coord,
    xyz_z: Coord,
}

type Coord = f64;

type GridCoord = i64;

// simpleSurf : T.Text -> Namelist
// simpleSurf surfId = Namelist "SURF" "" (M.fromList
//     [("ID", ParString surfId)
//     ])
//     (initialPos "Test Input")

// -- |Convert a @NamelistFile@ to an @FDSFile@.
// decodeNamelistFile : NamelistFile -> FDSFile
// decodeNamelistFile nmlFile =
//     foldl' decodeNamelist initFDSFile $ nmlFile_namelists nmlFile
//     where
//         inertSurf = decodeSurf (simpleSurf "INERT")
//         openSurf = decodeSurf (simpleSurf "OPEN")
//         hvacSurf = decodeSurf (simpleSurf "HVAC")
//         initFDSFile = FDSFile
//             { Head = Nothing
//             , Time = Nothing
//             , Dump = Nothing
//             , Misc = Nothing
//             , Meshes = []
//             , Reacs = []
//             , Devcs = []
//             , Matls = []
//             , Surfs = [inertSurf, openSurf, hvacSurf]
//             , Obsts = []
//             , Holes = []
//             , Hvacs = []
//             , Vents = []
//             , Bndfs = []
//             , Isofs = []
//             , Slcfs = []
//             , Ramps = []
//             , Props = []
//             , Parts = []
//             , Trnxs = []
//             , Trnys = []
//             , Trnzs = []
//             , unknownNamelists = []
//             }

// decodeNamelist : FDSFile -> Namelist -> FDSFile
// decodeNamelist fds#[derive(Clone, Debug)]
// pub struct nml nml_name nml of
//     "OBST" -> decodeObst fdsData nml
//     "VENT" -> decodeVent fdsData nml
//     "DEVC" -> decodeDevc fdsData nml
//     "PART" -> decodePart fdsData nml
//     "TIME" -> decodeTime fdsData nml
//     "PROP" -> decodeProp fdsData nml
//     "SURF" -> decodeSurfInto fdsData nml
//     "MESH" -> decodeMesh fdsData nml
//     "SLCF" -> decodeSlcf fdsData nml
//     "REAC" -> decodeReac fdsData nml
//     "HVAC" -> decodeHvac fdsData nml
//     "DUMP" -> decodeDump fdsData nml
//     "MISC" -> decodeMisc fdsData nml
//     "HEAD" -> decodeHead fdsData nml
//     _ -> decodeUnknown fdsData nml

// decodeUnknown : FDSFile -> Namelist -> FDSFile
// decodeUnknown fdsData nml =
//     fdsData { unknownNamelists = nml:(unknownNamelists fdsData)}

// decodeObst : FDSFile -> Namelist -> FDSFile
// decodeObst fdsData nml =
//     let obst = Obst
//             { ALLOW_VENT = fromMaybe True $ parToBool <$> getParameterMaybe nml "ALLOW_VENT"
//             , BNDF_FACE = fromMaybe (False, False, False, False, False, False)
//                $ parTo6 parToBool <$> getParameterMaybe nml "BNDF_FACE"
//             , BNDF_OBST = fromMaybe True $ parToBool <$> getParameterMaybe nml "BNDF_OBST"
//             , BULK_DENSITY = parToDouble <$> getParameterMaybe nml "BULK_DENSITY"
//             , COLOR = parToString <$> getParameterMaybe nml "COLOR"
//             , CTRL_ID = parToString <$> getParameterMaybe nml "CTRL_ID"
//             , DEVC_ID = parToString <$> getParameterMaybe nml "DEVC_ID"
//             , EVACUATION = fromMaybe False $ parToBool <$> getParameterMaybe nml "EVACUATION"
//             , FYI = parToString <$> getParameterMaybe nml "FYI"
//             , HT3D = fromMaybe False $ parToBool <$> getParameterMaybe nml "HT3D"
//             , ID = parToString <$> getParameterMaybe nml "ID"
//             , MATL_ID = parToString <$> getParameterMaybe nml "MATL_ID"
//             , MESH_ID = parToString <$> getParameterMaybe nml "MESH_ID"
//             , MULT_ID = parToString <$> getParameterMaybe nml "MULT_ID"
//             , OUTLINE = fromMaybe False $ parToBool <$> getParameterMaybe nml "OUTLINE"
//             , OVERLAY = fromMaybe True $ parToBool <$> getParameterMaybe nml "OVERLAY"
//             , PERMIT_HOLE = fromMaybe True $ parToBool <$> getParameterMaybe nml "PERMIT_HOLE"
//             , PROP_ID = parToString <$> getParameterMaybe nml "PROP_ID"
//             , REMOVABLE = fromMaybe True $ parToBool <$> getParameterMaybe nml "REMOVABLE"
//             , RGB = parToRGB <$> getParameterMaybe nml "RGB"
//             , SURF_ID = parToString <$> getParameterMaybe nml "SURF_ID"
//             , SURF_ID6 = parTo6String <$> getParameterMaybe nml "SURF_ID6"
//             , SURF_IDS = parTo3String <$> getParameterMaybe nml "SURF_IDS"
//             , TEXTURE_ORIGIN = XYZ 0 0 0
//             , THICKEN = fromMaybe False $ parToBool <$> getParameterMaybe nml "THICKEN"
//             , TRANSPARENCY = 1
//             , XB = fromMaybe (error "No XB value") $ getXBMaybe nml
//             }
//     in fdsData { Obsts = obst:(Obsts fdsData)}

// decodeVent : FDSFile -> Namelist -> FDSFile
// decodeVent fdsData nml =
//     let vent = Vent
//             { COLOR = parToString <$> getParameterMaybe nml "COLOR"
//             // , CTRL_ID : String
//             // , DEVC_ID : String
//             // , DYNAMIC_PRESSURE : f64
//             // , EVACUATION : bool
//             // , FYI : String
//             , ID = parToString <$> getParameterMaybe nml "ID"
//             // , IOR : i64
//             // , L_EDDY : f64
//             // , L_EDDY_IJ : Vec<i64>
//             // , MB : String
//             // , MESH_ID : String
//             // , MULT_ID : String
//             // , N_EDDY : i64
//             // , OUTLINE : bool
//             // , PBX : f64
//             // , PBY : f64
//             // , PBZ : f64
//             // , PRESSURE_RAMP : String
//             // , RADIUS : f64
//             // , REYNOLDS_STRESS : Vec<f64>
//             // , RGB : RGB
//             // , SPREAD_RATE : f64
//             , SURF_ID = parToString <$> getParameterMaybe nml "SURF_ID"
//             // , TEXTURE_ORIGIN : Vec<f64>
//             // , TMP_EXTERIOR : f64
//             // , TMP_EXTERIOR_RAMP : String
//             // , TRANSPARENCY : f64
//             // , UVW : Vec<f64>
//             // , VEL_RMS : f64
//             // -- , WIND : String
//             , XB = parToXB <$> getParameterMaybe nml "XB"
//             // , XYZ : XYZ
//             }
//     in fdsData { Vents = vent:(Vents fdsData)}

// parToList : (ParameterValue -> b) -> ParameterValue -> [b]
// parToList tranform (ParArray arr) = fmap tranform $ M.elems arr
// parToList _ xs = error $ show xs <> " is not an array"

// parToRGB : ParameterValue -> RGB
// parToRGB (ParArray arr) = case M.elems arr of
//     [a, b, c] -> RGB (parToInt a) (parToInt b) (parToInt c)
//     xs -> error $ show xs <> " is an invalid RGB array"
// parToRGB xs = error $ show xs <> " is not an array"

// parToXYZ : ParameterValue -> XYZ
// parToXYZ par =
//     let (x,y,z) = parTo3 parToDouble par
//     in XYZ x y z

// parToIJK : ParameterValue -> IJK
// parToIJK par =
//     let (i,j,k) = parTo3 parToInt par
//     in IJK i j k

// parToXB : ParameterValue -> XB
// parToXB par =
//     let (x1,x2,y1,y2,z1,z2) = parTo6 parToDouble par
//     in XB x1 x2 y1 y2 z1 z2

// parTo3String : ParameterValue -> (String, String, String)
// parTo3String (ParArray arr) =
//     let [a,b,c] = M.elems arr
//     in (parToString a, parToString b, parToString c)
// parTo3String xs = error $ show xs <> " is not an array"

// parTo2 : (ParameterValue -> b) -> ParameterValue -> (b, b)
// parTo2 tranform (ParArray arr) =
//     let [a, b] = fmap tranform $ M.elems arr
//     in (a, b)
// parTo2 _ xs = error $ show xs <> " is not an array"

// parTo3 : (ParameterValue -> c) -> ParameterValue -> (c, c, c)
// parTo3 tranform (ParArray arr) =
//     let [a, b, c] = fmap tranform $ M.elems arr
//     in (a, b, c)
// parTo3 _ xs = error $ show xs <> " is not an array"

// parTo6 : (ParameterValue -> f)
//             -> ParameterValue -> (f, f, f, f, f, f)
// parTo6 tranform (ParArray arr) = case fmap tranform $ M.elems arr of
//     [a,b,c,d,e,f] -> (a, b, c, d, e, f)
//     xs -> error $ "incorrect number of elements in array"
// parTo6 _ xs = error $ show xs <> " is not an array"

// parTo6String : ParameterValue
//     -> (String, String, String, String, String, String)
// parTo6String = parTo6 parToString

// decodeDevc : FDSFile -> Namelist -> FDSFile
// decodeDevc fdsData nml =
//     let
//         devcId = parToString <$> getParameterMaybe nml "ID"
//         devc = Devc
//             { BYPASS_FLOWRATE = fromMaybe 0 $ parToDouble <$> getParameterMaybe nml "BYPASS_FLOWRATE"
//             , CONVERSION_ADDEND = fromMaybe 0 $ parToDouble <$> getParameterMaybe nml "CONVERSION_ADDEND"
//             , CONVERSION_FACTOR = fromMaybe 1 $ parToDouble <$> getParameterMaybe nml "CONVERSION_FACTOR"
//             , COORD_FACTOR = fromMaybe 1 $ parToDouble <$> getParameterMaybe nml "COORD_FACTOR"
//             , CTRL_ID = Nothing
//             , DELAY = fromMaybe 0 $ parToDouble <$> getParameterMaybe nml "DELAY"
//             , DEPTH = fromMaybe 0 $ parToDouble <$> getParameterMaybe nml "DEPTH"
//             , DEVC_ID = parToString <$> getParameterMaybe nml "DEVC_ID"
//             , DRY = fromMaybe False $ parToBool <$> getParameterMaybe nml "DRY"
//             , DUCT_ID = parToString <$> getParameterMaybe nml "DUCT_ID"
//             , EVACUATION = fromMaybe False $ parToBool <$> getParameterMaybe nml "EVACUATION"
//             , FLOWRATE = fromMaybe 0 $ parToDouble <$> getParameterMaybe nml "FLOWRATE"
//             , FYI = parToString <$> getParameterMaybe nml "FYI"
//             , HIDE_COORDINATES = fromMaybe False $ parToBool <$> getParameterMaybe nml "HIDE_COORDINATES"
//             , ID = parToString <$> getParameterMaybe nml "ID"
//             , INITIAL_STATE = fromMaybe False $ parToBool <$> getParameterMaybe nml "INITIAL_STATE"
//             , INIT_ID = Nothing
//             , IOR = Nothing
//             , LATCH = True
//             , MATL_ID = Nothing
//             , NODE_ID = []
//             , NO_UPDATE_DEVC_ID = Nothing
//             , NO_UPDATE_CTRL_ID = Nothing
//             , ORIENTATION = fromMaybe (XYZ 0 0 1) $ parToXYZ <$> getParameterMaybe nml "ORIENTATION"
//             , ORIENTATION_NUMBER = fromMaybe 1 $ parToInt <$> getParameterMaybe nml "ORIENTATION_NUMBER"
//             , OUTPUT = fromMaybe True $ parToBool <$> getParameterMaybe nml "OUTPUT"
//             , PART_ID = parToString <$> getParameterMaybe nml "PART_ID"
//             , PIPE_INDEX = fromMaybe 1 $ parToInt <$> getParameterMaybe nml "PIPE_INDEX"
//             , POINTS = fromMaybe 1 $ parToInt <$> getParameterMaybe nml "POINTS"
//             , PROP_ID = parToString <$> getParameterMaybe nml "PROP_ID"
//             , QUANTITY = parToString <$> getParameterMaybe nml "QUANTITY"
//             , QUANTITY2 = parToString <$> getParameterMaybe nml "QUANTITY2"
//             , QUANTITY_RANGE = fromMaybe (-1e50,1e50) $ parTo2 parToDouble <$> getParameterMaybe nml "QUANTITY_RANGE"
//             , R_ID = parToString <$> getParameterMaybe nml "R_ID"
//             , REAC_ID = parToString <$> getParameterMaybe nml "REAC_ID"
//             , RELATIVE = fromMaybe False $ parToBool <$> getParameterMaybe nml "RELATIVE"
//             , ROTATION = fromMaybe 0 $ parToDouble <$> getParameterMaybe nml "ROTATION"
//             , SETPOINT = parToDouble <$> getParameterMaybe nml "SETPOINT"
//             , SMOOTHING_FACTOR = fromMaybe 0 $ parToDouble <$> getParameterMaybe nml "SMOOTHING_FACTOR"
//             , SPEC_ID = parToString <$> getParameterMaybe nml "SPEC_ID"
//             , STATISTICS = parToString <$> getParameterMaybe nml "STATISTICS"
//             , STATISTICS_START = fromMaybe 0 $ parToDouble <$> getParameterMaybe nml "STATISTICS_START" -- TODO: T_BEGIN is the default, how do we get that?
//             , SURF_ID = parToString <$> getParameterMaybe nml "SURF_ID"
//             , TIME_AVERAGED = fromMaybe True $ parToBool <$> getParameterMaybe nml "TIME_AVERAGED"
//             , TIME_HISTORY = fromMaybe False $ parToBool <$> getParameterMaybe nml "TIME_HISTORY"
//             , TRIP_DIRECTION = fromMaybe 1 $ parToInt <$> getParameterMaybe nml "TRIP_DIRECTION"
//             , UNITS = parToString <$> getParameterMaybe nml "UNITS"
//             , VELO_INDEX = fromMaybe 0 $ parToInt <$> getParameterMaybe nml "VELO_INDEX"
//             , XB = parToXB <$> getParameterMaybe nml "XB"
//             , XYZ = parToXYZ <$> getParameterMaybe nml "XYZ"
//             , X_ID = (\n->n<>"-x") <$> devcId
//             , Y_ID = (\n->n<>"-y") <$> devcId
//             , Z_ID = (\n->n<>"-z") <$> devcId
//             , XYZ_UNITS = "m"
//             }
//     in fdsData { Devcs = devc:(Devcs fdsData)}

// decodeMesh : FDSFile -> Namelist -> FDSFile
// decodeMesh fdsData nml =
//     let
//         // devcId = parToString <$> getParameterMaybe nml "ID"
//         mesh = Mesh
//             { ID = parToString <$> getParameterMaybe nml "ID"
//             , XB = fromMaybe (XB 0 1 0 1 0 1) $ parToXB <$> getParameterMaybe nml "XB"
//             , IJK = fromMaybe (IJK 10 10 10) $ parToIJK <$> getParameterMaybe nml "IJK"
//             , COLOR = fromMaybe "BLACK" $ parToString <$> getParameterMaybe nml "COLOR"
//             , CYLINDRICAL = fromMaybe False $ parToBool <$> getParameterMaybe nml "CYLINDRICAL"
//             , EVACUATION = fromMaybe False $ parToBool <$> getParameterMaybe nml "EVACUATION"
//             , EVAC_HUMANS = fromMaybe False $ parToBool <$> getParameterMaybe nml "EVAC_HUMANS"
//             , EVAC_Z_OFFSET = fromMaybe 1 $ parToDouble <$> getParameterMaybe nml "EVAC_Z_OFFSET"
//             , FYI = parToString <$> getParameterMaybe nml "FYI"
//             , LEVEL = fromMaybe 1 $ parToInt <$> getParameterMaybe nml "LEVEL"
//             , MPI_PROCESS = parToInt <$> getParameterMaybe nml "LEVEL"
//             , MULT_ID = parToString <$> getParameterMaybe nml "FYI"
//             , RGB = fromMaybe (RGB 0 0 0) $ parToRGB <$> getParameterMaybe nml "RGB"
//             , N_THREADS = parToInt <$> getParameterMaybe nml "N_THREADS"
//             // , PERIODIC_MESH_IDS : [Text]
//             }
//     in fdsData { Meshes = mesh:(Meshes fdsData)}

// decodePart : FDSFile -> Namelist -> FDSFile
// decodePart fdsData nml =
//     let
//         part = Part
//             { AGE = fromMaybe 1e5 $ parToDouble <$> getParameterMaybe nml "AGE"
//             , BREAKUP = fromMaybe False $ parToBool <$> getParameterMaybe nml "BREAKUP"
//             , BREAKUP_CNF_RAMP_ID = parToString <$> getParameterMaybe nml "BREAKUP_CNF_RAMP_ID"
//             , BREAKUP_DISTRIBUTION = fromMaybe "ROSIN.." $ parToString <$> getParameterMaybe nml "BREAKUP_CNF_RAMP_ID"
//             , BREAKUP_GAMMA_D = fromMaybe 2.4 $ parToDouble <$> getParameterMaybe nml "BREAKUP_GAMMA_D"
//             , BREAKUP_RATIO = fromMaybe (3/7) $ parToDouble <$> getParameterMaybe nml "BREAKUP_RATIO"
//             , BREAKUP_SIGMA_D = parToDouble <$> getParameterMaybe nml "BREAKUP_SIGMA_D"
//             , CHECK_DISTRIBUTION = fromMaybe False $ parToBool <$> getParameterMaybe nml "CHECK_DISTRIBUTION"
//             , CNF_RAMP_ID = parToString <$> getParameterMaybe nml "CNF_RAMP_ID"
//             , COLOR = fromMaybe "BLACK" $ parToString <$> getParameterMaybe nml "COLOR"
//             , COMPLEX_REFRACTIVE_INDEX = fromMaybe 0.01 $ parToDouble <$> getParameterMaybe nml "COMPLEX_REFRACTIVE_INDEX"
//             , CTRL_ID = parToString <$> getParameterMaybe nml "CTRL_ID"
//             , DENSE_VOLUME_FRACTION = fromMaybe 1e-5 $ parToDouble <$> getParameterMaybe nml "DENSE_VOLUME_FRACCTION"
//             , DEVC_ID = parToString <$> getParameterMaybe nml "DEVC_ID"
//             , DIAMETER = parToDouble <$> getParameterMaybe nml "DIAMETER"
//             , DISTRIBUTION = fromMaybe "ROSIN..." $ parToString <$> getParameterMaybe nml "DISTRIBUTION"
//             , DRAG_COEFFICIENT = fromMaybe [] $ parToList parToDouble <$> getParameterMaybe nml "DRAG_COEFFICIENT"
//             , DRAG_LAW = fromMaybe "SPHERE" $ parToString <$> getParameterMaybe nml "DRAG_LAW"
//             , FREE_AREA_FRACTION = parToDouble <$> getParameterMaybe nml "FREE_AREA_FRACTION"
//             , FYI = parToString <$> getParameterMaybe nml "FYI"
//             , GAMMA_D  = fromMaybe 2.4 $ parToDouble <$> getParameterMaybe nml "GAMMA_D"
//             , HEAT_OF_COMBUSTION = parToDouble <$> getParameterMaybe nml "HEAT_OF_COMBUSTION"
//             , HORIZONTAL_VELOCITY  = fromMaybe 0.2 $ parToDouble <$> getParameterMaybe nml "HORIZONTAL_VELOCITY"
//             , ID = parToString <$> getParameterMaybe nml "ID"
//             // , INITIAL_TEMPERATURE : f64 -- TMPA
//             , MASSLESS = fromMaybe False $ parToBool <$> getParameterMaybe nml "MASSLESS"
//             , MAXIMUM_DIAMETER = fromMaybe (1/0 {- Infinity -}) $ parToDouble <$> getParameterMaybe nml "MAXIMUM_DIAMETER"
//             , MINIMUM_DIAMETER = fromMaybe 20 $ parToDouble <$> getParameterMaybe nml "MINIMUM_DIAMETER"
//             , MONODISPERSE = fromMaybe False $ parToBool <$> getParameterMaybe nml "MONODISPERSE"
//             , N_STRATA = fromMaybe 6 $ parToInt <$> getParameterMaybe nml "N_STRATA"
//             // , ORIENTATION : Vec<f64>
//             // , PERMEABILITY : Vec<f64>
//             , PERIODIC_X = fromMaybe False $ parToBool <$> getParameterMaybe nml "PERIODIC_X"
//             , PERIODIC_Y = fromMaybe False $ parToBool <$> getParameterMaybe nml "PERIODIC_Y"
//             , PERIODIC_Z = fromMaybe False $ parToBool <$> getParameterMaybe nml "PERIODIC_Z"
//             , POROUS_VOLUME_FRACTION = parToDouble <$> getParameterMaybe nml "POROUS_VOLUME_FRACTION"
//             , PROP_ID = parToString <$> getParameterMaybe nml "PROP_ID"
//             // , QUANTITIES : Vec<String>
//             // , QUANTITIES_SPEC_ID : Vec<String>
//             , RADIATIVE_PROPERTY_TABLE = parToDouble <$> getParameterMaybe nml "RADIATIVE_PROPERTY_TABLE"
//             , REAL_REFRACTIVE_INDEX = fromMaybe 1.33 $ parToDouble <$> getParameterMaybe nml "REAL_REFRACTIVE_INDEX"
//             , RGB = parToRGB <$> getParameterMaybe nml "RGB"
//             , RUNNING_AVERAGE_FACTOR = fromMaybe 0.5 $ parToDouble <$> getParameterMaybe nml "RUNNING_AVERAGE_FACTOR"
//             , SAMPLING_FACTOR = fromMaybe 1 $ parToInt <$> getParameterMaybe nml "SAMPLING_FACTOR"
//             , SECOND_ORDER_PARTICLE_TRANSPORT = fromMaybe False $ parToBool <$> getParameterMaybe nml "SECOND_ORDER_PARTICLE_TRANSPORT"
//             , SIGMA_D = parToDouble <$> getParameterMaybe nml "SIGMA_D"
//             , SPEC_ID = parToString <$> getParameterMaybe nml "SPEC_ID"
//             , STATIC = fromMaybe False $ parToBool <$> getParameterMaybe nml "STATIC"
//             , SURFACE_TENSION = fromMaybe 7.28e-2 $ parToDouble <$> getParameterMaybe nml "SURFACE_TENSION"
//             , SURF_ID = parToString <$> getParameterMaybe nml "SURF_ID"
//             // , TARGET_ONLY : bool
//             , TURBULENT_DISPERSION = fromMaybe False $ parToBool <$> getParameterMaybe nml "AGE"
//             , VERTICAL_VELOCITY = fromMaybe 0.5 $ parToDouble <$> getParameterMaybe nml "VERTICAL_VELOCITY"
//             }
//     in fdsData { Parts = part:(Parts fdsData)}

// decodeSurfInto : FDSFile -> Namelist -> FDSFile
// decodeSurfInto fdsData nml =
//     let surf = decodeSurf nml
//     in fdsData { Surfs = surf:(Surfs fdsData)}

// decodeSurf : Namelist -> Surf
// decodeSurf nml = Surf
//             { ADIABATIC = fromMaybe False $ parToBool <$> getParameterMaybe nml "ADIABATIC"
//             , AUTO_IGNITION_TEMPERATURE = fromMaybe (-273) $ parToDouble <$> getParameterMaybe nml "AUTO_IGNITION_TEMPERATURE"
//             , BACKING = fromMaybe "EXPOSED" $ parToString <$> getParameterMaybe nml "BACKING"
//             , BURN_AWAY = fromMaybe False $ parToBool <$> getParameterMaybe nml "BACKING"
//             , CELL_SIZE_FACTOR = fromMaybe 1.0 $ parToDouble <$> getParameterMaybe nml "CELL_SIZE_FACTOR"
//             , C_FORCED_CONSTANT = fromMaybe 0.0 $ parToDouble <$> getParameterMaybe nml "C_FORCED_CONSTANT"
//             , C_FORCED_PR_EXP = fromMaybe 0.0 $ parToDouble <$> getParameterMaybe nml "C_FORCED_PR_EXP"
//             , C_FORCED_RE = fromMaybe 0.0 $ parToDouble <$> getParameterMaybe nml "C_FORCED_RE"
//             , C_FORCED_RE_EXP = fromMaybe 0.0 $ parToDouble <$> getParameterMaybe nml "C_FORCED_RE_EXP"
//             , C_HORIZONTAL = fromMaybe 1.52 $ parToDouble <$> getParameterMaybe nml "C_HORIZONTAL"
//             , C_VERTICAL = fromMaybe 1.31 $ parToDouble <$> getParameterMaybe nml "C_VERTICAL"
//             , COLOR = parToString <$> getParameterMaybe nml "COLOR"
//             // , CONVECTION_LENGTH_SCALE : f64
//             // , CONVECTIVE_HEAT_FLUX : f64
//             // , CONVERT_VOLUME_TO_MASS : bool
//             // , DEFAULT : bool
//             // , DT_INSERT : f64
//             // , EMISSIVITY : f64
//             // , EMISSIVITY_BACK : f64
//             // , EVAC_DEFAULT : bool
//             // , EXTERNAL_FLUX : f64
//             // , E_COEFFICIENT : f64
//             // , FIRELINE_MLR_MAX : f64
//             // , FREE_SLIP : bool
//             // , FYI : String
//             // , GEOMETRY : String
//             // , HEAT_OF_VAPORIZATION : f64
//             // , HEAT_TRANSFER_COEFFICIENT : f64
//             // , HEAT_TRANSFER_COEFFICIENT_BACK : f64
//             // , HEAT_TRANSFER_MODEL : String
//             , HRRPUA = parToDouble <$> getParameterMaybe nml "HRRPUA"
//             // , HT3D : bool
//             , ID = parToString <$> getParameterMaybe nml "ID"
//             // , IGNITION_TEMPERATURE : f64
//             // , INNER_RADIUS : f64
//             // , INTERNAL_HEAT_SOURCE : Vec<f64>
//             // , LAYER_DIVIDE : f64
//             // , LEAK_PATH : Vec<i64>
//             // , LENGTH : f64
//             , MASS_FLUX = Nothing -- parToDouble <$> getParameterMaybe nml "MASS_FLUX"
//             , MASS_FLUX_TOTAL = parToDouble <$> getParameterMaybe nml "MASS_FLUX_TOTAL"
//             , MASS_FLUX_VAR = parToDouble <$> getParameterMaybe nml "MASS_FLUX_VAR"
//             // , MASS_FRACTION : Vec<f64>
//             // , MASS_TRANSFER_COEFFICIENT : f64
//             // , MATL_ID : Vec<String>
//             // , MATL_MASS_FRACTION : Vec<f64>
//             // , MINIMUM_LAYER_THICKNESS : f64
//             , MLRPUA = parToDouble <$> getParameterMaybe nml "MLRPUA"
//             // , N_LAYER_CELLS_MAX : Vec<i64>
//             // , NET_HEAT_FLUX : f64
//             // , NO_SLIP : bool
//             // , NPPC : i64
//             // , PARTICLE_MASS_FLUX : f64
//             // , PART_ID : String
//             // , PLE : f64
//             // , PROFILE : String
//             // , RADIUS : f64
//             // , RAMP_EF : String
//             // , RAMP_MF : Vec<String>
//             // , RAMP_PART : String
//             , RAMP_Q = parToString <$> getParameterMaybe nml "RAMP_Q"
//             , RAMP_T = parToString <$> getParameterMaybe nml "RAMP_T"
//             // , RAMP_T_I : Option<String>
//             // , RAMP_V : Option<String>
//             // , RAMP_V_X : Option<String>
//             // , RAMP_V_Y : Option<String>
//             // , RAMP_V_Z : Option<String>
//             // , RGB : RGB
//             // , ROUGHNESS : f64
//             // , SPEC_ID : String
//             // , SPREAD_RATE : f64
//             // , STRETCH_FACTOR : Vec<f64>
//             // , TAU_EF : f64
//             // , TAU_MF : f64
//             // , TAU_PART : f64
//             , TAU_Q = fromMaybe 1 $ parToDouble <$> getParameterMaybe nml "TAU_Q"
//             // , TAU_T : f64
//             // , TAU_V : f64
//             // , TEXTURE_HEIGHT : f64
//             // , TEXTURE_MAP : String
//             // , TEXTURE_WIDTH : f64
//             // , TGA_ANALYSIS : bool
//             // , TGA_FINAL_TEMPERATURE : f64
//             // , TGA_HEATING_RATE : f64
//             // , THICKNESS : Vec<f64>
//             // , TMP_BACK : f64
//             // , TMP_FRONT : f64
//             // , TMP_INNER : Vec<f64>
//             // , TRANSPARENCY : f64
//             // , VEGETATION : bool
//             // , VEGETATION_ARRHENIUS_DEGRAD : bool
//             // , VEGETATION_CDRAG : f64
//             // , VEGETATION_CHAR_FRACTION : f64
//             // , VEGETATION_ELEMENT_DENSITY : i64
//             // , VEGETATION_GROUND_TEMP : f64
//             // , VEGETATION_HEIGHT : f64
//             // , VEGETATION_INITIAL_TEMP : f64
//             // , VEGETATION_LAYERS : i64
//             // , VEGETATION_LINEAR_DEGRAD : bool
//             // , VEGETATION_LOAD : f64
//             // , VEGETATION_LSET_IGNITE_TIME : f64
//             // , VEG_LSET_QCON : f64
//             // , VEGETATION_MOISTURE : f64
//             // , VEGETATION_NO_BURN : bool
//             // , VEGETATION_SVRATIO : i64
//             // , VEG_LEVEL_SET_SPREAD : bool
//             // , VEG_LSET_ROS_BACK : f64
//             // , VEG_LSET_ROS_FLANK : f64
//             // , VEG_LSET_ROS_HEAD : f64
//             // , VEG_LSET_WIND_EXP : f64
//             // , VEG_LSET_SIGMA : f64
//             // , VEG_LSET_HT : f64
//             // , VEG_LSET_BETA : f64
//             // , VEG_LSET_ELLIPSE : f64
//             // , VEG_LSET_TAN2 : bool
//             // , VEG_LSET_ELLIPSE_HEAD : f64
//             , VEL = parToDouble <$> getParameterMaybe nml "VEL"
//             // , VEL_BULK : f64
//             // , VEL_GRAD : f64
//             , VEL_T = parTo2 parToDouble <$> getParameterMaybe nml "VEL_T"
//             , VOLUME_FLOW = parToDouble <$> getParameterMaybe nml "VOLUME_FLOW"
//             // , WIDTH : f64
//             // , XYZ : XYZ
//             // , Z0 : f64
//             }

// decodeProp : FDSFile -> Namelist -> FDSFile
// decodeProp fdsData nml =
//     let
//         prop = Prop
//             { ACTIVATION_OBSCURATION = fromMaybe 3.24 $ parToDouble <$> getParameterMaybe nml "ACTIVATION_OBSCURATION"
//             , ACTIVATION_TEMPERATURE = fromMaybe 74 $ parToDouble <$> getParameterMaybe nml "ACTIVATION_TEMPERATURE"
//             // , ALPHA_C : f64
//             // , ALPHA_E : f64
//             // , BEAD_DENSITY : f64
//             // , BEAD_DIAMETER : f64
//             // , BEAD_EMISSIVITY : f64
//             // , BEAD_HEAT_TRANSFER_COEFFICIENT : f64
//             // , BEAD_SPECIFIC_HEAT : f64
//             // , BETA_C : f64
//             // , BETA_E : f64
//             // -- , FED_ACTIVITY : String
//             // , CHARACTERISTIC_VELOCITY : f64
//             // , C_FACTOR : f64
//             // , DENSITY : f64
//             // , DIAMETER : f64
//             // , DROPLET_VELOCITY : f64
//             // , EMISSIVITY : f64
//             // , FLOW_RAMP : String
//             // , FLOW_RATE : f64
//             // , FLOW_TAU : f64
//             // , FYI : String
//             // , GAUGE_EMISSIVITY : f64
//             // , GAUGE_TEMPERATURE : f64
//             // , HEAT_TRANSFER_COEFFICIENT : f64
//             , ID = parToString <$> getParameterMaybe nml "ID"
//             // , INITIAL_TEMPERATURE : f64
//             // , K_FACTOR : f64
//             // , LENGTH : f64
//             // , MASS_FLOW_RATE : f64
//             // , OFFSET : f64
//             // , OPERATING_PRESSURE : f64
//             // , ORIFICE_DIAMETER : f64
//             // , P0 : String
//             // , PARTICLES_PER_SECOND : i64
//             // , PARTICLE_VELOCITY : f64
//             // , PART_ID : String
//             // , PDPA_END : f64
//             // , PDPA_HISTOGRAM : bool
//             // , PDPA_HISTOGRAM_LIMITS : Vec<f64>
//             // , PDPA_HISTOGRAM_NBINS : i64
//             // , PDPA_HISTOGRAM_CUMULATIVE : bool
//             // , PDPA_INTEGRATE : bool
//             // , PDPA_M : i64
//             // , PDPA_N : i64
//             // , PDPA_NORMALIZE : bool
//             // , PDPA_RADIUS : f64
//             // , PDPA_START : f64
//             // , PRESSURE_RAMP : String
//             // -- , PX : String
//             // -- , PXX : String
//             , QUANTITY = parToString <$> getParameterMaybe nml "QUANTITY"
//             // , RTI : f64
//             // , SMOKEVIEW_ID : Vec<String>
//             // , SMOKEVIEW_PARAMETERS : Vec<String>
//             // , SPEC_ID : String
//             // , SPRAY_ANGLE : Vec<f64>
//             // , SPRAY_PATTERN_BETA : f64
//             // , SPRAY_PATTERN_MU : f64
//             // , SPRAY_PATTERN_SHAPE : String
//             // , SPRAY_PATTERN_TABLE : String
//             // , VELOCITY_COMPONENT : i64
//             // -- , DROPLET_VELOCITY : String
//             }
//     in fdsData { Props = prop:(Props fdsData)}

// decodeSlcf : FDSFile -> Namelist -> FDSFile
// decodeSlcf fdsData nml =
//     let
//         slcf = Slcf
//             { CELL_CENTERED = fromMaybe False $ parToBool <$> getParameterMaybe nml "CELL_CENTERED"
//             , EVACUATION = fromMaybe False $ parToBool <$> getParameterMaybe nml "EVACUATION"
//             // -- , FACE_CENTERED : String
//             // -- , FIRE_LINE : String
//             // , FYI : String
//             , ID = parToString <$> getParameterMaybe nml "ID"
//             // , IOR : i64
//             // , LEVEL_SET_FIRE_LINE : String
//             // , MAXIMUM_VALUE : f64
//             // , MESH_NUMBER : i64
//             // , MINIMUM_VALUE : f64
//             // , PART_ID : String
//             , PBX = parToDouble <$> getParameterMaybe nml "PBX"
//             , PBY = parToDouble <$> getParameterMaybe nml "PBY"
//             , PBZ = parToDouble <$> getParameterMaybe nml "PBZ"
//             // -- , PROP_ID : String
//             , QUANTITY = parToString <$> getParameterMaybe nml "QUANTITY"
//             , QUANTITY2 = parToString <$> getParameterMaybe nml "QUANTITY2"
//             // , REAC_ID : String
//             // -- , SLICETYPE : String
//             , SPEC_ID = parToString <$> getParameterMaybe nml "SPEC_ID"
//             // , VECTOR : bool
//             // , VELO_INDEX : i64
//             // , XB : XB
//             }
//     in fdsData { Slcfs = slcf:(Slcfs fdsData)}

// decodeHvac : FDSFile -> Namelist -> FDSFile
// decodeHvac fdsData nml =
//     let
//         hvac = Hvac
//             { AIRCOIL_ID = parToString <$> getParameterMaybe nml "AIRCOIL_ID"
//             // , AMBIENT : bool
//             // , AREA : f64
//             // , CLEAN_LOSS : f64
//             // , COOLANT_SPECIFIC_HEAT : f64
//             // , COOLANT_MASS_FLOW : f64
//             // , COOLANT_TEMPERATURE : f64
//             // , CTRL_ID : String
//             // , DAMPER : bool
//             // , DEVC_ID : String
//             // , DIAMETER : f64
//             // , DUCT_ID : Vec<String>
//             // , DUCT_INTERP_TYPE : String
//             // , EFFICIENCY : Vec<f64>
//             // , FAN_ID : String
//             // , FILTER_ID : String
//             // , FIXED_Q : Vec<f64>
//             // , ID : String
//             // , LEAK_ENTHALPY : bool
//             // , LENGTH : f64
//             // , LOADING : Vec<f64>
//             // , LOADING_MULTIPLIER : Vec<f64>
//             // , LOSS : Vec<f64>
//             // , MASS_FLOW : f64
//             // , MAX_FLOW : f64
//             // , MAX_PRESSURE : f64
//             // , N_CELLS : i64
//             // , NODE_ID : Vec<String>
//             // , PERIMETER : f64
//             // , RAMP_ID : String
//             // , RAMP_LOSS : String
//             // , REVERSE : bool
//             // , ROUGHNESS : f64
//             // , SPEC_ID : String
//             // , TAU_AC : f64
//             // , TAU_FAN : f64
//             // , TAU_VF : f64
//             // , TYPE_ID : String
//             , VENT_ID = parToString <$> getParameterMaybe nml "VENT_ID"
//             , VENT2_ID = parToString <$> getParameterMaybe nml "VENT2_ID"
//             // , VOLUME_FLOW : f64
//             // , XYZ : XYZ
//             }
//     in fdsData { Hvacs = hvac:(Hvacs fdsData)}

// decodeReac : FDSFile -> Namelist -> FDSFile
// decodeReac fdsData nml =
//     let
//         reac = Reac
//             { A = parToDouble <$> getParameterMaybe nml "A"
//             // -- , ALT_REAC_ID : String
//             // , AUTO_IGNITION_TEMPERATURE : f64
//             , C = fromMaybe 0 $ parToDouble <$> getParameterMaybe nml "C"
//             // , CHECK_ATOM_BALANCE : bool
//             , CO_YIELD = fromMaybe 0 $ parToDouble <$> getParameterMaybe nml "CO_YIELD"
//             // , CRITICAL_FLAME_TEMPERATURE : f64
//             // , E : f64
//             , EPUMO2 = fromMaybe 13100 $ parToDouble <$> getParameterMaybe nml "EPUMO2"
//             // -- , K : String
//             // , EQUATION : String
//             // , FIXED_MIX_TIME : f64
//             // -- , FLAME_SPEED : String
//             // -- , FLAME_SPEED_EXPONENT : String
//             // -- , FLAME_SPEED_TEMPERATURE : String
//             // , FORMULA : String
//             // , FUEL : String
//             // , FUEL_RADCAL_ID : String
//             // -- , FWD_ID : String
//             , FYI = parToString <$> getParameterMaybe nml "FYI"
//             , H = fromMaybe 0 $ parToDouble <$> getParameterMaybe nml "H"
//             // , HEAT_OF_COMBUSTION : f64
//             , ID = parToString <$> getParameterMaybe nml "ID"
//             // , IDEAL : bool
//             , N = fromMaybe 0 $ parToDouble <$> getParameterMaybe nml "N"
//             // , NU : Vec<f64>
//             // , N_S : Vec<f64>
//             // , N_T : f64
//             , O = fromMaybe 0 $ parToDouble <$> getParameterMaybe nml "O"
//             // -- , ODE_SOLVER : String
//             // , RADIATIVE_FRACTION : f64
//             // , RAMP_CHI_R : String
//             // -- , RAMP_FS : String
//             // , REAC_ATOM_ERROR : f64
//             // , REAC_MASS_ERROR : f64
//             // -- , REVERSE : String
//             , SOOT_H_FRACTION = fromMaybe 0.1 $ parToDouble <$> getParameterMaybe nml "SOOT_H_FRACTION"
//             , SOOT_YIELD = fromMaybe 0 $ parToDouble <$> getParameterMaybe nml "SOOT_YIELD"
//             // , SPEC_ID_N_S : Vec<String>
//             // , SPEC_ID_NU : Vec<String>
//             // -- , TABLE_FS : String
//             // -- , TAU_CHEM : String
//             // -- , TAU_FLAME : String
//             // , THIRD_BODY : bool
//             // -- , TURBULENT_FLAME_SPEED_ALPHA : String
//             // -- , TURBULENT_FLAME_SPEED_EXPONENT : String
//             // -- , Y_P_MIN_EDC : String
//             }
//     in fdsData { Reacs = reac:(Reacs fdsData)}

// decodeMisc : FDSFile -> Namelist -> FDSFile
// decodeMisc fdsData nml =
//     let
//         misc = Misc
//             { ALLOW_SURFACE_PARTICLES = fromMaybe False $ parToBool <$> getParameterMaybe nml "ALLOW_SURFACE_PARTICLES"
//             // , ALLOW_UNDERSIDE_PARTICLES : bool
//             // , ASSUMED_GAS_TEMPERATURE : f64
//             // , ASSUMED_GAS_TEMPERATURE_RAMP : String
//             // , BAROCLINIC : bool
//             // , BNDF_DEFAULT : bool
//             // , CC_IBM : bool
//             // , CNF_CUTOFF : f64
//             // , CFL_MAX : f64
//             // , CFL_MIN : f64
//             // , CFL_VELOCITY_NORM : i64
//             // , CHECK_HT : bool
//             // , CHECK_REALIZABILITY : bool
//             // , CHECK_VN : bool
//             // , CLIP_MASS_FRACTION : bool
//             // , COMPUTE_VISCOSITY_TWICE : bool
//             // , COMPUTE_ZETA_SOURCE_TERM : bool
//             // , CONSTANT_H_SOLID : bool
//             // , CONSTANT_SPECIFIC_HEAT_RATIO : bool
//             // , CORIOLIS_VECTOR : Vec<f64>
//             // , CORRECT_SUBGRID_TEMPERATURE : bool
//             // , COUPLED_1D3D_HEAT_TRANSFER : bool
//             // , C_DEARDORFF : f64
//             // , C_RNG : f64
//             // , C_RNG_CUTOFF : f64
//             // , C_SMAGORINSKY : f64
//             // , C_VREMAN : f64
//             // , DNS : bool
//             // , DRAG_CFL_MAX : f64
//             // , DT_MEAN_FORCING : f64
//             // , ENTHALPY_TRANSPORT : bool
//             // , EVACUATION_DRILL : bool
//             // , EVACUATION_MC_MODE : bool
//             // , EVAC_PRESSURE_ITERATIONS : i64
//             // , EVAC_SURF_DEFAULT : String
//             // , EVAC_TIME_ITERATIONS : i64
//             // , EVAPORATION : bool
//             // -- , EXCHANGE_EDGES : String
//             // , EXTERNAL_BOUNDARY_CORRECTION : bool
//             // , EXTINCTION_MODEL : String
//             // , HVAC_PRES_RELAX : f64
//             // , HT3D_TEST : i64
//             // , FDS5_OPTIONS : bool
//             // , FLUX_LIMITER : i64
//             // , FORCE_VECTOR : Vec<f64>
//             // , FREEZE_VELOCITY : bool
//             // , FYI : String
//             // , GAMMA : f64
//             // , GRAVITATIONAL_DEPOSITION : bool
//             // , GRAVITATIONAL_SETTLING : bool
//             // , GROUND_LEVEL : f64
//             // , GVEC : Vec<f64>
//             // , DT_HVAC : f64
//             // , H_F_REFERENCE_TEMPERATURE : f64
//             // , HRRPUV_MAX_SMV : f64
//             // , HUMIDITY : f64
//             // , HVAC_MASS_TRANSPORT : bool
//             // , IBLANK_SMV : bool
//             // , IMMERSED_BOUNDARY_METHOD : i64
//             // , INITIAL_UNMIXED_FRACTION : f64
//             // -- , KINETIC_ENERGY_SOURCE : String
//             // , LAPSE_RATE : f64
//             // , LES_FILTER_WIDTH : String
//             // , MAX_CHEMISTRY_ITERATIONS : i64
//             // , MAX_LEAK_PATHS : i64
//             , MAXIMUM_VISIBILITY = fromMaybe 30 $ parToDouble <$> getParameterMaybe nml "MAXIMUM_VISIBILITY"
//             // , MEAN_FORCING : [Bool]
//             // , MPI_TIMEOUT : f64
//             // , N_FIXED_CHEMISTRY_SUBSTEPS : i64
//             // , NEAR_WALL_TURBULENCE_MODEL : String
//             // -- , NEW_MOMENTUM_NUDGING : String
//             // -- , NEW_OPEN_BOUNDARY : String
//             // , NOISE : bool
//             // , NOISE_VELOCITY : f64
//             // , NO_EVACUATION : bool
//             // , NO_RAMPS : bool
//             // -- , NORTHANGLE : String
//             // , OVERWRITE : bool
//             // , PARTICLE_CFL_MAX : f64
//             // , PARTICLE_CFL_MIN : f64
//             // , PARTICLE_CFL : bool
//             // , PERIODIC_TEST : i64
//             // -- , PROFILING : String
//             // , POROUS_FLOOR : bool
//             // -- , POTENTIAL_TEMPERATURE_CORRECTION : String
//             // , PR : f64
//             // , PROCESS_ALL_MESHES : bool
//             // , PROJECTION : bool
//             // , P_INF : f64
//             // -- , RAMP_FVX_T : String
//             // -- , RAMP_FVY_T : String
//             // -- , RAMP_FVZ_T : String
//             // , RAMP_GX : String
//             // , RAMP_GY : String
//             // , RAMP_GZ : String
//             // , RAMP_U0 : String
//             // , RAMP_U0_T : String
//             // , RAMP_V0 : String
//             // , RAMP_V0_T : String
//             // , RAMP_W0 : String
//             // , RAMP_W0_T : String
//             // , RAMP_U0_Z : String
//             // , RAMP_V0_Z : String
//             // , RAMP_W0_Z : String
//             // -- , RADIATION : String
//             // , RESEARCH_MODE : bool
//             // , RESTART : bool
//             // , RESTART_CHID : String
//             // , RICHARDSON_ERROR_TOLERANCE : f64
//             // , RUN_AVG_FAC : f64
//             // , SC : f64
//             // , SECOND_ORDER_INTERPOLATED_BOUNDARY : bool
//             // , SECOND_ORDER_PARTICLE_TRANSPORT : bool
//             // , SHARED_FILE_SYSTEM : bool
//             // -- , SLIP_CONDITION : String
//             // , SMOKE_ALBEDO : f64
//             // , SOLID_PHASE_ONLY : bool
//             // -- , SOOT_OXIDATION : String
//             // -- , SPONGE_LAYER_DISTANCE : String
//             // , STRATIFICATION : bool
//             // , SUPPRESSION : bool
//             // -- , SURF_DEFAULT : String
//             // -- , TEMPERATURE_DEPENDENT_REACTION : String
//             // -- , TENSOR_DIFFUSIVITY : String
//             // , TERRAIN_CASE : bool
//             // , TERRAIN_IMAGE : String
//             // -- , TEST_FILTER_QUADRATURE : String
//             // , TEXTURE_ORIGIN : Vec<f64>
//             // , THERMOPHORETIC_DEPOSITION : bool
//             // , THICKEN_OBSTRUCTIONS : bool
//             // -- , TRANSPORT_UNMIXED_FRACTION : String
//             // -- , TRANSPORT_ZETA_SCHEME : String
//             // , TMPA : f64
//             // , TURBULENCE_MODEL : String
//             // , TURBULENT_DEPOSITION : bool
//             // -- , TURB_INIT_CLOCK : String
//             // , U0 : f64
//             // , UVW_FILE : String
//             // , V0 : f64
//             // , VEG_LEVEL_SET_COUPLED : bool
//             // , VEG_LEVEL_SET_UNCOUPLED : bool
//             // , VERBOSE : f64
//             , VISIBILITY_FACTOR = fromMaybe 3 $ parToDouble <$> getParameterMaybe nml "VISIBILITY_FACTOR"
//             // , VN_MAX : f64
//             // , VN_MIN : f64
//             // , Y_CO2_INFTY : f64
//             // , Y_O2_INFTY : f64
//             // , W0 : f64
//             // -- , WD_PROPS : String
//             // -- , WIND_BOUNDARY : String
//             // -- , WIND_ONLY : String
//             }
//     in fdsData { Misc = (Just misc)}

// decodeTime : FDSFile -> Namelist -> FDSFile
// decodeTime fdsData nml =
//     let
//         time = Time
//             { DT = parToDouble <$> getParameterMaybe nml "DT"
//             // , EVAC_DT_FLOWFIELD : f64
//             // , EVAC_DT_STEADY_STATE : f64
//             , FYI = parToString <$> getParameterMaybe nml "FYI"
//             // , LIMITING_DT_RATIO : f64
//             // , LOCK_TIME_STEP : bool
//             // , RESTRICT_TIME_STEP : bool
//             , T_BEGIN = fromMaybe 0 $ parToDouble <$> getParameterMaybe nml "T_BEGIN"
//             , T_END = fromMaybe 1 $ parToDouble <$> getParameterMaybe nml "T_END"
//             // , T_END_GEOM : f64
//             // , TIME_SHRINK_FACTOR : f64
//             // , WALL_INCREMENT : i64
//             // , WALL_INCREMENT_HT3D : i64
//             // , TWFIN : f64
//             }
//     in fdsData { Time = (Just time)}

// decodeHead : FDSFile -> Namelist -> FDSFile
// decodeHead fdsData nml =
//     let
//         head = Head
//             { head_CHID = parToString <$> getParameterMaybe nml "CHID"
//             , head_FYI = parToString <$> getParameterMaybe nml "FYI"
//             , head_TITLE = parToString <$> getParameterMaybe nml "TITLE"
//             }
//     in fdsData { Head = (Just head)}

// decodeDump : FDSFile -> Namelist -> FDSFile
// decodeDump fdsData nml =
//     let
//         dump = Dump
//             { CLIP_RESTART_FILES = fromMaybe True $ parToBool <$> getParameterMaybe nml "CLIP_RESTART_FILES"
//             // , COLUMN_DUMP_LIMIT : bool
//             // , CTRL_COLUMN_LIMIT : i64
//             // , DEVC_COLUMN_LIMIT : i64
//             // , DT_BNDE : f64
//             // , DT_BNDF : f64
//             // , DT_CPU : f64
//             // , DT_CTRL : f64
//             // , DT_DEVC : f64
//             // , DT_DEVC_LINE : f64
//             // , DT_FLUSH : f64
//             // , DT_GEOM : f64
//             // , DT_HRR : f64
//             // , DT_ISOF : f64
//             // , DT_MASS : f64
//             // , DT_PART : f64
//             // , DT_PL3D : f64
//             // , DT_PROF : f64
//             , DT_RESTART = fromMaybe 1000000 $ parToDouble <$> getParameterMaybe nml "DT_RESTART"
//             // , DT_SL3D : f64
//             // , DT_SLCF : f64
//             // , EB_PART_FILE : bool
//             // , FLUSH_FILE_BUFFERS : bool
//             // , GEOM_DIAG : bool
//             // , MASS_FILE : bool
//             // , MAXIMUM_PARTICLES : i64
//             // , MMS_TIMER : f64
//             , NFRAMES = fromMaybe 1000 $ parToInt <$> getParameterMaybe nml "NFRAMES"
//             // , PLOT3D_PART_ID : Vec<String>
//             // , PLOT3D_QUANTITY : Vec<String>
//             // , PLOT3D_SPEC_ID : Vec<String>
//             // , PLOT3D_VELO_INDEX : Vec<i64>
//             // , RENDER_FILE : String
//             // , SIG_FIGS : i64
//             // , SIG_FIGS_EXP : i64
//             // , SMOKE3D : bool
//             // , SMOKE3D_QUANTITY : String
//             // , SMOKE3D_SPEC_ID : String
//             // , STATUS_FILES : bool
//             // , SUPPRESS_DIAGNOSTICS : bool
//             // , UVW_TIMER : Vec<f64>
//             // , VELOCITY_ERROR_FILE : bool
//             // , WRITE_XYZ : bool
//             }
//     in fdsData { Dump = (Just dump)}
