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
// use namelist::Namelist;
// use namelist::NamelistFile;
use namelist::*;
use crate::xb::*;
use crate::xb::HasXB;

/// The Haskell data type representation of an FDS input script. The first items
/// (such as head and time) are single occurrence items. As any of these items
/// may or may not occur they are given maybe types. The other items may occur
/// zero, one or many times, and are therefore given a list type. There is
/// provision for storing namelists that are not understood for the purposes of
/// forward compatibility.
#[derive(Clone, Debug)]
pub struct FDSFile {
    pub head: Option<Head>,
    pub time: Option<Time>,
    pub dump: Option<Dump>,
    pub misc: Option<Misc>,
    pub meshes: Vec<Mesh>,
    pub reacs: Vec<Reac>,
    pub devcs: Vec<Devc>,
    pub matls: Vec<Matl>,
    pub surfs: Vec<Surf>,
    pub obsts: Vec<Obst>,
    pub holes: Vec<Hole>,
    pub hvacs: Vec<Hvac>,
    pub vents: Vec<Vent>,
    pub bndfs: Vec<Bndf>,
    pub isofs: Vec<Isof>,
    pub slcfs: Vec<Slcf>,
    pub ramps: Vec<Ramp>,
    pub props: Vec<Prop>,
    pub parts: Vec<Part>,
    pub trnxs: Vec<Trnx>,
    pub trnys: Vec<Trny>,
    pub trnzs: Vec<Trnz>,
    pub unknown_namelists: Vec<Namelist>,
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
    prop_id: Option<String>,
    recount_drip: bool,
    quantity: Option<String>,
    spec_id: Option<String>,
    statistics: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Bnde {
    cell_centered: bool,
    fyi: Option<String>,
    part_id: Option<String>,
    prop_id: Option<String>,
    quantity: Option<String>,
    spec_id: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Clip {
    fyi: Option<String>,
    maximum_density: f64,
    maximum_mass_fraction: Vec<Vec<f64>>,
    maximum_temperature: f64,
    minimum_density: f64,
    minimum_mass_fraction: Vec<Vec<f64>>,
    minimum_temperature: f64,
}

#[derive(Clone, Debug)]
pub struct Ctrl {
    constant: f64,
    // , CYCLES : String
    // , CYCLE_TIME : String
    delay: f64,
    differential_gain: f64,
    evacuation: bool,
    function_type: String,
    id: String,
    initial_state: bool,
    integral_gain: f64,
    input_id: Vec<String>,
    latch: bool,
    n: i64,
    on_bound: String,
    proportional_gain: f64,
    ramp_id: String,
    setpoint: f64,
    target_value: f64,
    trip_direction: i64,
}

#[derive(Clone, Debug)]
pub struct Csvf {
    csvfile: String,
    uvwfile: String,
}

#[derive(Clone, Debug)]
pub struct Devc {
    // bypass_flowrate: f64,
    // conversion_addend: f64,
    // conversion_factor: f64,
    // coord_factor: f64,
    // ctrl_id: Option<String>,
    // delay: f64,
    // depth: f64,
    // devc_id: Option<String>,
    // dry: bool,
    // duct_id: Option<String>,
    // evacuation: bool,
    // flowrate: f64,
    // fyi: Option<String>,
    // hide_coordinates: bool,
    // id: Option<String>,
    // initial_state: bool,
    // init_id: Option<String>,
    // ior: Option<i64>,
    // latch: bool,
    // matl_id: Option<String>,
    // node_id: Vec<String>,
    // no_update_devc_id: Option<String>,
    // no_update_ctrl_id: Option<String>,
    // orientation: XYZ,
    // orientation_number: i64,
    // output: bool,
    // part_id: Option<String>,
    // pipe_index: i64,
    // points: i64,
    pub prop_id: Option<String>,
    // quantity: Option<String>,
    // quantity2: Option<String>,
    // quantity_range: (f64, f64),
    // r_id: Option<String>,
    // reac_id: Option<String>,
    // relative: bool,
    // rotation: f64,
    // setpoint: Option<f64>,
    // smoothing_factor: f64,
    // spec_id: Option<String>,
    // statistics: Option<String>,
    // statistics_start: f64,
    // surf_id: Option<String>,
    // time_averaged: bool,
    // time_history: bool,
    // trip_direction: i64,
    // units: Option<String>,
    // velo_index: i64,
    // xb: Option<XB>,
    pub xyz: Option<XYZ>,
    // x_id: Option<String>,
    // y_id: Option<String>,
    // z_id: Option<String>,
    // xyz_units: String,
}

#[derive(Clone, Debug)]
pub struct Dump {
    clip_restart_files: bool,
    column_dump_limit: bool,
    ctrl_column_limit: i64,
    devc_column_limit: i64,
    dt_bnde: f64,
    dt_bndf: f64,
    dt_cpu: f64,
    dt_ctrl: f64,
    dt_devc: f64,
    dt_devc_line: f64,
    dt_flush: f64,
    dt_geom: f64,
    dt_hrr: f64,
    dt_isof: f64,
    dt_mass: f64,
    dt_part: f64,
    dt_pl3d: f64,
    dt_prof: f64,
    dt_restart: f64,
    dt_sl3d: f64,
    dt_slcf: f64,
    eb_part_file: bool,
    flush_file_buffers: bool,
    geom_diag: bool,
    mass_file: bool,
    maximum_particles: i64,
    mms_timer: f64,
    nframes: i64,
    plot3d_part_id: Vec<String>,
    plot3d_quantity: Vec<String>,
    plot3d_spec_id: Vec<String>,
    plot3d_velo_index: Vec<i64>,
    render_file: String,
    sig_figs: i64,
    sig_figs_exp: i64,
    smoke3d: bool,
    smoke3d_quantity: String,
    smoke3d_spec_id: String,
    status_files: bool,
    suppress_diagnostics: bool,
    uvw_timer: Vec<f64>,
    velocity_error_file: bool,
    write_xyz: bool,
}

#[derive(Clone, Debug)]
pub struct Hole {
    color: String,
    ctrl_id: String,
    devc_id: String,
    evacuation: bool,
    fyi: Option<String>,
    id: String,
    mesh_id: String,
    mult_id: String,
    rgb: RGB,
    transparency: f64,
    xb: XB,
}

#[derive(Clone, Debug)]
pub struct Hvac {
    aircoil_id: Option<String>,
    ambient: bool,
    area: f64,
    clean_loss: f64,
    coolant_specific_heat: f64,
    coolant_mass_flow: f64,
    coolant_temperature: f64,
    ctrl_id: String,
    damper: bool,
    devc_id: String,
    diameter: f64,
    duct_id: Vec<String>,
    duct_interp_type: String,
    efficiency: Vec<f64>,
    fan_id: String,
    filter_id: String,
    fixed_q: Vec<f64>,
    id: String,
    leak_enthalpy: bool,
    length: f64,
    loading: Vec<f64>,
    loading_multiplier: Vec<f64>,
    loss: Vec<f64>,
    mass_flow: f64,
    max_flow: f64,
    max_pressure: f64,
    n_cells: i64,
    node_id: Vec<String>,
    perimeter: f64,
    ramp_id: String,
    ramp_loss: String,
    reverse: bool,
    roughness: f64,
    spec_id: String,
    tau_ac: f64,
    tau_fan: f64,
    tau_vf: f64,
    type_id: String,
    vent_id: Option<String>,
    vent2_id: Option<String>,
    volume_flow: f64,
    xyz: XYZ,
}

#[derive(Clone, Debug)]
pub struct Init {
    auto_ignition_temperature: f64,
    cell_centered: bool,
    ctrl_id: String,
    density: f64,
    devc_id: String,
    diameter: f64,
    dt_insert: f64,
    dx: f64,
    dy: f64,
    dz: f64,
    height: f64,
    hrrpuv: f64,
    id: String,
    mass_fraction: Vec<f64>,
    mass_per_time: f64,
    mass_per_volume: f64,
    mult_id: String,
    n_particles: i64,
    n_particles_per_cell: i64,
    part_id: String,
    radius: f64,
    shape: String,
    spec_id: Vec<String>,
    temperature: f64,
    uvw: Vec<f64>,
    xb: XB,
    xyz: XYZ,
    particle_weight_factor: f64,
    number_initial_particles: i64,
}

#[derive(Clone, Debug)]
pub struct Isof {
    fyi: Option<String>,
    quantity: String,
    spec_id: String,
    value: f64,
    velo_index: i64,
}

#[derive(Clone, Debug)]
pub struct Matl {
    a: Vec<f64>,
    absorption_coefficient: f64,
    boiling_temperature: f64,
    color: String,
    conductivity: f64,
    conductivity_ramp: String,
    density: f64,
    e: Vec<f64>,
    emissivity: f64,
    fyi: Option<String>,
    heating_rate: Vec<f64>,
    heat_of_combustion: Vec<f64>,
    heat_of_reaction: Vec<f64>,
    id: String,
    matl_id: Vec<String>,
    nu_matl: Vec<f64>,
    nu_spec: Vec<f64>,
    n_reactions: i64,
    n_s: Vec<f64>,
    n_t: Vec<f64>,
    n_o2: Vec<f64>,
    pcr: Vec<bool>,
    pyrolysis_range: Vec<f64>,
    reference_rate: Vec<f64>,
    reference_temperature: Vec<f64>,
    rgb: RGB,
    specific_heat: f64,
    specific_heat_ramp: String,
    spec_id: Vec<String>,
    threshold_sign: Vec<f64>,
    threshold_temperature: Vec<f64>,
    // , POROSITY : String
    allow_shrinking: bool,
    allow_swelling: bool,
    gas_diffusion_depth: Vec<f64>,
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
    pub id: Option<String>,
    pub xb: XB,
    pub ijk: IJK,
    // pub color: String,
    // pub cylindrical: bool,
    // pub evacuation: bool,
    // pub evac_humans: bool,
    // pub evac_z_offset: f64,
    // pub fyi: Option<String>,
    // pub level: i64,
    // pub mpi_process: Option<i64>,
    // pub mult_id: Option<String>,
    // pub rgb: RGB,
    // pub n_threads: Option<i64>, // , PERIODIC_MESH_IDS : [Text]
}

impl Mesh {
    pub fn resolution(&self) -> (f64, f64, f64) {
        let ijk = self.ijk;
        let (dx, dy, dz) = self.dimensions();
        (dx/(ijk.i as f64), dy/(ijk.j as f64), dz/(ijk.k as f64))
    }
    pub fn dimensions(&self) -> (f64, f64, f64) {
        let xb = self.xb();
        let dx = xb.x2 - xb.x1;
        let dy = xb.y2 - xb.y1;
        let dz = xb.z2 - xb.z1;
        (dx, dy, dz)
    }
    pub fn cells(&self) -> u64 {
        (self.ijk.i * self.ijk.j * self.ijk.k) as u64
    }
}


impl HasXB for Mesh {
    fn xb(&self) -> XB {
        self.xb.clone()
    }
}

impl HasXB for &Mesh {
    fn xb(&self) -> XB {
        self.xb.clone()
    }
}

impl HasXB for &mut Mesh {
    fn xb(&self) -> XB {
        self.xb.clone()
    }
}


#[derive(Clone, Debug)]
pub struct Misc {
    // agglomeration: bool,
    // aerosol_al2o3: bool,
    // allow_surface_particles: bool,
    // allow_underside_particles: bool,
    // assumed_gas_temperature: f64,
    // assumed_gas_temperature_ramp: String,
    // baroclinic: bool,
    // bndf_default: bool,
    // cc_ibm: bool,
    // cnf_cutoff: f64,
    // cfl_max: f64,
    // cfl_min: f64,
    // cfl_velocity_norm: i64,
    // check_ht: bool,
    // check_realizability: bool,
    // check_vn: bool,
    // clip_mass_fraction: bool,
    // compute_viscosity_twice: bool,
    // compute_zeta_source_term: bool,
    // constant_h_solid: bool,
    // constant_specific_heat_ratio: bool,
    // coriolis_vector: Vec<f64>,
    // correct_subgrid_temperature: bool,
    // coupled_1d3d_heat_transfer: bool,
    // c_deardorff: f64,
    // c_rng: f64,
    // c_rng_cutoff: f64,
    // c_smagorinsky: f64,
    // c_vreman: f64,
    // dns: bool,
    // drag_cfl_max: f64,
    // dt_mean_forcing: f64,
    // enthalpy_transport: bool,
    // evacuation_drill: bool,
    // evacuation_mc_mode: bool,
    // evac_pressure_iterations: i64,
    // evac_surf_default: String,
    // evac_time_iterations: i64,
    // evaporation: bool,
    // // , EXCHANGE_EDGES : String
    // external_boundary_correction: bool,
    // extinction_model: String,
    // hvac_pres_relax: f64,
    // ht3d_test: i64,
    // fds5_options: bool,
    // flux_limiter: i64,
    // force_vector: Vec<f64>,
    // freeze_velocity: bool,
    // fyi: Option<String>,
    // gamma: f64,
    // gravitational_deposition: bool,
    // gravitational_settling: bool,
    // ground_level: f64,
    // gvec: Vec<f64>,
    // dt_hvac: f64,
    // h_f_reference_temperature: f64,
    // hrrpuv_max_smv: f64,
    // humidity: f64,
    // hvac_mass_transport: bool,
    // iblank_smv: bool,
    // immersed_boundary_method: i64,
    // initial_unmixed_fraction: f64,
    // // , KINETIC_ENERGY_SOURCE : String
    // lapse_rate: f64,
    // les_filter_width: String,
    // max_chemistry_iterations: i64,
    // max_leak_paths: i64,
    pub maximum_visibility: f64,
    // mean_forcing: Vec<bool>,
    // mpi_timeout: f64,
    // n_fixed_chemistry_substeps: i64,
    // near_wall_turbulence_model: String,
    // // , NEW_MOMENTUM_NUDGING : String
    // // , NEW_OPEN_BOUNDARY : String
    // noise: bool,
    // noise_velocity: f64,
    // no_evacuation: bool,
    // no_ramps: bool,
    // // , NORTHANGLE : String
    // overwrite: bool,
    // particle_cfl_max: f64,
    // particle_cfl_min: f64,
    // particle_cfl: bool,
    // periodic_test: i64,
    // // , PROFILING : String
    // porous_floor: bool,
    // // , POTENTIAL_TEMPERATURE_CORRECTION : String
    // pr: f64,
    // process_all_meshes: bool,
    // projection: bool,
    // p_inf: f64,
    // // , RAMP_FVX_T : String
    // // , RAMP_FVY_T : String
    // // , RAMP_FVZ_T : String
    // ramp_gx: String,
    // ramp_gy: String,
    // ramp_gz: String,
    // ramp_u0: String,
    // ramp_u0_t: String,
    // ramp_v0: String,
    // ramp_v0_t: String,
    // ramp_w0: String,
    // ramp_w0_t: String,
    // ramp_u0_z: String,
    // ramp_v0_z: String,
    // ramp_w0_z: String,
    // // , RADIATION : String
    // research_mode: bool,
    // restart: bool,
    // restart_chid: String,
    // richardson_error_tolerance: f64,
    // run_avg_fac: f64,
    // sc: f64,
    // second_order_interpolated_boundary: bool,
    // second_order_particle_transport: bool,
    // shared_file_system: bool,
    // // , SLIP_CONDITION : String
    // smoke_albedo: f64,
    // solid_phase_only: bool,
    // // , SOOT_OXIDATION : String
    // // , SPONGE_LAYER_DISTANCE : String
    // stratification: bool,
    // suppression: bool,
    // // , SURF_DEFAULT : String
    // // , TEMPERATURE_DEPENDENT_REACTION : String
    // // , TENSOR_DIFFUSIVITY : String
    // terrain_case: bool,
    // terrain_image: String,
    // // , TEST_FILTER_QUADRATURE : String
    // texture_origin: Vec<f64>,
    // thermophoretic_deposition: bool,
    // thicken_obstructions: bool,
    // // , TRANSPORT_UNMIXED_FRACTION : String
    // // , TRANSPORT_ZETA_SCHEME : String
    // tmpa: f64,
    // turbulence_model: String,
    // turbulent_deposition: bool,
    // // , TURB_INIT_CLOCK : String
    // u0: f64,
    // uvw_file: String,
    // v0: f64,
    // veg_level_set_coupled: bool,
    // veg_level_set_uncoupled: bool,
    // verbose: f64,
    pub visibility_factor: f64,
    // vn_max: f64,
    // vn_min: f64,
    // y_co2_infty: f64,
    // y_o2_infty: f64,
    // w0: f64,
    // // , WD_PROPS : String
    // // , WIND_BOUNDARY : String
    // // , WIND_ONLY : String
}

#[derive(Clone, Debug)]
pub struct Mult {
    dx: f64,
    dxb: Vec<f64>,
    dx0: f64,
    dy: f64,
    dyb: Vec<f64>,
    dy0: f64,
    dz: f64,
    dzb: Vec<f64>,
    dz0: f64,
    id: String,
    i_lower: i64,
    i_upper: i64,
    j_lower: i64,
    j_upper: i64,
    k_lower: i64,
    k_upper: i64,
    n_lower: i64,
    n_upper: i64,
}

#[derive(Clone, Debug)]
pub struct Obst {
    // allow_vent: bool,
    // bndf_face: (bool, bool, bool, bool, bool, bool),
    // bndf_obst: bool,
    // bulk_density: Option<f64>,
    // color: Option<String>,
    // ctrl_id: Option<String>,
    // devc_id: Option<String>,
    // evacuation: bool,
    // fyi: Option<String>,
    // ht3d: bool,
    id: Option<String>,
    // matl_id: Option<String>,
    // mesh_id: Option<String>,
    // mult_id: Option<String>,
    // // , NOTERRAIN : bool
    // outline: bool,
    // overlay: bool,
    // permit_hole: bool,
    // prop_id: Option<String>,
    // removable: bool,
    // rgb: Option<RGB>,
    surf_id: Option<String>,
    // surf_id6: Option<(String, String, String, String, String, String)>,
    // surf_ids: Option<(String, String, String)>,
    // texture_origin: XYZ,
    // thicken: bool,
    // transparency: f64,
    xb: XB,
}

#[derive(Clone, Debug)]
pub struct Part {
    age: f64,
    breakup: bool,
    breakup_cnf_ramp_id: Option<String>,
    breakup_distribution: String,
    breakup_gamma_d: f64,
    breakup_ratio: f64,
    breakup_sigma_d: Option<f64>,
    check_distribution: bool,
    cnf_ramp_id: Option<String>,
    color: String,
    complex_refractive_index: f64,
    ctrl_id: Option<String>,
    dense_volume_fraction: f64,
    devc_id: Option<String>,
    diameter: Option<f64>,
    distribution: String,
    drag_coefficient: Vec<f64>,
    drag_law: String,
    free_area_fraction: Option<f64>,
    fyi: Option<String>,
    gamma_d: f64,
    heat_of_combustion: Option<f64>,
    horizontal_velocity: f64,
    id: Option<String>,
    initial_temperature: f64,
    massless: bool,
    maximum_diameter: f64,
    minimum_diameter: f64,
    monodisperse: bool,
    n_strata: i64,
    orientation: Vec<f64>,
    permeability: Vec<f64>,
    periodic_x: bool,
    periodic_y: bool,
    periodic_z: bool,
    porous_volume_fraction: Option<f64>,
    prop_id: Option<String>,
    quantities: Vec<String>,
    quantities_spec_id: Vec<String>,
    radiative_property_table: Option<f64>,
    real_refractive_index: f64,
    rgb: Option<RGB>,
    running_average_factor: f64,
    sampling_factor: i64,
    second_order_particle_transport: bool,
    sigma_d: Option<f64>,
    spec_id: Option<String>,
    static_: bool,
    surface_tension: f64,
    surf_id: Option<String>,
    target_only: bool,
    turbulent_dispersion: bool,
    vertical_velocity: f64,
}

#[derive(Clone, Debug)]
pub struct Pres {
    check_poisson: bool,
    fishpak_bc: Vec<i64>,
    // , GLMAT_SOLVER : String
    iteration_suspend_factor: f64,
    // , LAPLACE_PRESSURE_CORRECTION : String
    max_pressure_iterations: i64,
    pressure_relax_time: f64,
    pressure_tolerance: f64,
    relaxation_factor: f64,
    scarc_method: String,
    scarc_krylov: String,
    scarc_multigrid: String,
    scarc_smooth: String,
    scarc_precon: String,
    scarc_coarse: String,
    scarc_initial: String,
    scarc_accuracy: f64,
    scarc_debug: String,
    scarc_multigrid_cycle: String,
    scarc_multigrid_level: String,
    scarc_multigrid_coarsening: String,
    scarc_multigrid_iterations: i64,
    scarc_multigrid_accuracy: f64,
    scarc_multigrid_interpol: String,
    scarc_krylov_iterations: i64,
    scarc_krylov_accuracy: f64,
    scarc_smooth_iterations: i64,
    scarc_smooth_accuracy: f64,
    scarc_smooth_omega: String,
    scarc_precon_iterations: i64,
    scarc_precon_accuracy: f64,
    scarc_precon_omega: String,
    scarc_coarse_iterations: i64,
    scarc_coarse_accuracy: f64,
    solver: String,
    suspend_pressure_iterations: i64,
    velocity_tolerance: f64,
}

#[derive(Clone, Debug)]
pub struct Prof {
    format_index: i64,
    fyi: String,
    id: String,
    ior: i64,
    quantity: String,
    xyz: XYZ,
}

#[derive(Clone, Debug)]
pub struct Prop {
    pub activation_obscuration: f64,
    pub activation_temperature: f64,
    // alpha_c: f64,
    // alpha_e: f64,
    // bead_density: f64,
    // bead_diameter: f64,
    // bead_emissivity: f64,
    // bead_heat_transfer_coefficient: f64,
    // bead_specific_heat: f64,
    // beta_c: f64,
    // beta_e: f64,
    // // , FED_ACTIVITY : String
    // characteristic_velocity: f64,
    // c_factor: f64,
    // density: f64,
    // diameter: f64,
    // droplet_velocity: f64,
    // emissivity: f64,
    // flow_ramp: String,
    pub flow_rate: f64,
    // flow_tau: f64,
    // fyi: Option<String>,
    // gauge_emissivity: f64,
    // gauge_temperature: f64,
    // heat_transfer_coefficient: f64,
    // id: Option<String>,
    // initial_temperature: f64,
    // k_factor: f64,
    // length: f64,
    // mass_flow_rate: f64,
    // offset: f64,
    // operating_pressure: f64,
    // orifice_diameter: f64,
    // p0: String,
    // particles_per_second: i64,
    // particle_velocity: f64,
    pub part_id: String,
    // pdpa_end: f64,
    // pdpa_histogram: bool,
    // pdpa_histogram_limits: Vec<f64>,
    // pdpa_histogram_nbins: i64,
    // pdpa_histogram_cumulative: bool,
    // pdpa_integrate: bool,
    // pdpa_m: i64,
    // pdpa_n: i64,
    // pdpa_normalize: bool,
    // pdpa_radius: f64,
    // pdpa_start: f64,
    // pressure_ramp: String, // , PX : String
    // // , PXX : String
    pub quantity: Option<String>,
    pub rti: f64,
    // smokeview_id: Vec<String>,
    // smokeview_parameters: Vec<String>,
    // spec_id: String,
    // spray_angle: Vec<f64>,
    // spray_pattern_beta: f64,
    // spray_pattern_mu: f64,
    // spray_pattern_shape: String,
    // spray_pattern_table: String,
    // velocity_component: i64, // , DROPLET_VELOCITY : String
}

#[derive(Clone, Debug)]
pub struct Radi {
    angle_increment: i64,
    band_limits: Vec<f64>,
    c_max: f64,
    c_min: f64,
    initial_radiation_iterations: i64,
    kappa0: f64,
    nmieang: i64,
    number_radiation_angles: i64,
    path_length: f64,
    radiation: bool,
    radiation_iterations: i64,
    // , RADIATIVE_FRACTION : String
    radtmp: f64,
    // , RTE_SOURCE_CORRECTION : String
    time_step_increment: i64,
    wide_band_model: bool,
    mie_minimum_diameter: f64,
    mie_maximum_diameter: f64,
    mie_ndg: i64,
    number_initial_iterations: i64,
    qr_clip: f64,
}

#[derive(Clone, Debug)]
pub struct Ramp {
    id: String,
    entries: Vec<RampEntry>,
}

#[derive(Clone, Debug)]
pub struct RampEntry {
    ctrl_id: String,
    devc_id: String,
    f: f64,
    fyi: Option<String>,
    number_interpolation_points: i64,
    t: f64,
    x: f64,
    z: f64,
}

#[derive(Clone, Debug)]
pub struct Reac {
    pub a: Option<f64>,
    // , ALT_REAC_ID : String
    pub auto_ignition_temperature: f64,
    pub c: f64,
    pub check_atom_balance: bool,
    pub co_yield: f64,
    pub critical_flame_temperature: f64,
    pub e: f64,
    pub epumo2: f64,
    // , K : String
    pub equation: String,
    pub fixed_mix_time: f64,
    // , FLAME_SPEED : String
    // , FLAME_SPEED_EXPONENT : String
    // , FLAME_SPEED_TEMPERATURE : String
    pub formula: String,
    pub fuel: String,
    pub fuel_radcal_id: String,
    // , FWD_ID : String
    pub fyi: Option<String>,
    pub h: f64,
    pub heat_of_combustion: f64,
    pub id: Option<String>,
    pub ideal: bool,
    pub n: f64,
    pub nu: Vec<f64>,
    pub n_s: Vec<f64>,
    pub n_t: f64,
    pub o: f64,
    // , ODE_SOLVER : String
    pub radiative_fraction: f64,
    pub ramp_chi_r: String,
    // , RAMP_FS : String
    pub reac_atom_error: f64,
    pub reac_mass_error: f64,
    // , REVERSE : String
    pub soot_h_fraction: f64,
    pub soot_yield: f64,
    pub spec_id_n_s: Vec<String>,
    pub spec_id_nu: Vec<String>,
    // , TABLE_FS : String
    // , TAU_CHEM : String
    // , TAU_FLAME : String
    pub third_body: bool,
    // , TURBULENT_FLAME_SPEED_ALPHA : String
    // , TURBULENT_FLAME_SPEED_EXPONENT : String
    // , Y_P_MIN_EDC : String
}

#[derive(Clone, Debug)]
pub struct Slcf {
    agl_slice: String,
    cell_centered: bool,
    evacuation: bool, // , FACE_CENTERED : String
    // , FIRE_LINE : String
    fyi: Option<String>,
    id: Option<String>,
    ior: i64,
    level_set_fire_line: String,
    maximum_value: f64,
    mesh_number: i64,
    minimum_value: f64,
    part_id: String,
    pbx: Option<f64>,
    pby: Option<f64>,
    pbz: Option<f64>, // , PROP_ID : String
    quantity: Option<String>,
    quantity2: Option<String>,
    reac_id: String, // , SLICETYPE : String
    spec_id: Option<String>,
    vector: bool,
    velo_index: i64,
    xb: XB,
}

#[derive(Clone, Debug)]
pub struct Spec {
    aerosol: bool,
    alias: String,
    background: bool, // , COPY_LUMPED : String
    conductivity: f64,
    conductivity_solid: f64,
    density_liquid: f64,
    density_solid: f64,
    diffusivity: f64,
    enthalpy_of_formation: f64,
    epsilonklj: f64,
    fic_concentration: f64,
    fld_lethal_dose: f64,
    formula: String,
    fyi: Option<String>,
    heat_of_vaporization: f64,
    h_v_reference_temperature: f64,
    id: String,
    lumped_component_only: bool,
    mass_extinction_coefficient: f64,
    mass_fraction: Vec<f64>,
    mass_fraction_0: f64, // , MAX_DIAMETER : String
    mean_diameter: f64,
    melting_temperature: f64, // , MIN_DIAMETER : String
    mw: f64,                  // , N_BINS : String
    pr_gas: f64,
    primitive: bool,
    radcal_id: String,
    ramp_cp: String,
    ramp_cp_l: String,
    ramp_d: String,
    ramp_g_f: String,
    ramp_k: String,
    ramp_mu: String,
    reference_enthalpy: f64,
    reference_temperature: f64,
    sigmalj: f64,
    spec_id: Vec<String>,
    specific_heat: f64,
    specific_heat_liquid: f64,
    vaporization_temperature: f64,
    viscosity: f64,
    volume_fraction: Vec<f64>,
}

#[derive(Clone, Debug)]
pub struct Surf {
    pub adiabatic: bool,
    pub auto_ignition_temperature: f64,
    // pub backing: String,
    // pub burn_away: bool,
    // pub cell_size_factor: f64,
    // pub c_forced_constant: f64,
    // pub c_forced_pr_exp: f64,
    // pub c_forced_re: f64,
    // pub c_forced_re_exp: f64,
    // pub c_horizontal: f64,
    // pub c_vertical: f64,
    pub color: Option<String>,
    // pub convection_length_scale: f64,
    // pub convective_heat_flux: Option<f64>,
    // pub convert_volume_to_mass: bool,
    // pub default: bool,
    // pub dt_insert: f64,
    // pub emissivity: f64,
    // pub emissivity_back: Option<f64>,
    // pub evac_default: bool,
    // pub external_flux: f64,
    // pub extinction_temperature: f64,
    // pub free_slip: bool,
    pub fyi: Option<String>,
    // pub geometry: String,
    // pub heat_of_vaporization: f64,
    // pub heat_transfer_coefficient: f64,
    // pub heat_transfer_coefficient_back: f64,
    // pub heat_transfer_model: String,
    pub hrrpua: Option<f64>,
    // pub ht3d: bool,
    pub id: Option<String>,
    // pub ignition_temperature: f64,
    // pub inner_radius: f64,
    // pub internal_heat_source: Vec<f64>,
    // pub layer_divide: f64,
    // pub leak_path: Vec<i64>,
    // pub length: f64,
    // pub mass_flux: Option<Vec<f64>>,
    // pub mass_flux_total: Option<f64>,
    // pub mass_flux_var: Option<f64>,
    // pub mass_fraction: Vec<f64>,
    // pub mass_transfer_coefficient: f64,
    // pub matl_id: Vec<String>,
    // pub matl_mass_fraction: Vec<f64>,
    // pub minimum_layer_thickness: f64,
    pub mlrpua: Option<f64>,
    // , N_CELLS_MAX : String
    // n_layer_cells_max: Vec<i64>,
    // net_heat_flux: f64,
    // no_slip: bool,
    // nppc: i64,
    // particle_mass_flux: f64,
    // part_id: String,
    // ple: f64,
    // profile: String,
    // radius: f64,
    // ramp_ef: String,
    // ramp_mf: Vec<String>,
    // ramp_part: String,
    // ramp_q: Option<String>,
    // ramp_t: Option<String>,
    // ramp_t_i: Option<String>,
    // ramp_v: Option<String>,
    // ramp_v_x: Option<String>,
    // ramp_v_y: Option<String>,
    // ramp_v_z: Option<String>,
    // rgb: RGB,
    // roughness: f64,
    // spec_id: String,
    // spread_rate: f64,
    // stretch_factor: f64,
    // tau_ef: f64,
    // tau_mf: f64,
    // tau_part: f64,
    // tau_q: f64,
    // tau_t: f64,
    // tau_v: f64,
    // texture_height: f64,
    // texture_map: String,
    // texture_width: f64,
    // tga_analysis: bool,
    // tga_final_temperature: f64,
    // tga_heating_rate: f64,
    // thickness: Vec<f64>,
    // tmp_back: f64,
    // tmp_front: f64,
    // tmp_inner: Vec<f64>,
    // transparency: f64,
    // vegetation: bool,
    // vegetation_arrhenius_degrad: bool,
    // vegetation_cdrag: f64,
    // vegetation_char_fraction: f64,
    // vegetation_element_density: i64,
    // vegetation_ground_temp: f64,
    // vegetation_height: f64,
    // vegetation_initial_temp: f64,
    // vegetation_layers: i64,
    // vegetation_linear_degrad: bool,
    // vegetation_load: f64,
    // vegetation_lset_ignite_time: f64,
    // veg_lset_qcon: f64,
    // vegetation_moisture: f64,
    // vegetation_no_burn: bool,
    // vegetation_svratio: i64,
    // veg_level_set_spread: bool,
    // veg_lset_ros_back: f64,
    // veg_lset_ros_flank: f64,
    // veg_lset_ros_head: f64,
    // veg_lset_wind_exp: f64,
    // veg_lset_sigma: f64,
    // veg_lset_ht: f64,
    // veg_lset_beta: f64,
    // veg_lset_ellipse: f64,
    // veg_lset_tan2: bool,
    // veg_lset_ellipse_head: f64,
    // vel: Option<f64>,
    // vel_bulk: f64,
    // vel_grad: f64,
    // vel_t: Option<(f64, f64)>,
    // volume_flow: Option<f64>,
    // width: f64,
    // xyz: XYZ,
    // z0: f64,
    // , ZETA_FRONT : String
    // , EXTERNAL_FLUX_RAMP : String
    // , TAU_EXTERNAL_FLUX : String
    // , VOLUME_FLUX : String
}

impl Default for Surf {
    fn default() -> Self {
        Surf {
            adiabatic: true,
            auto_ignition_temperature: -273_f64,
            // backing: "EXPOSED".to_string(),
            // burn_away: false,
            // cell_size_factor: 1_f64,
            // c_forced_constant: 0_f64,
            // c_forced_pr_exp: 0_f64,
            // c_forced_re: 0_f64,
            // c_forced_re_exp: 0_f64,
            // c_horizontal: 1.52_f64,
            // c_vertical: 1.31_f64,
            color: None,
            // convection_length_scale: 1_f64,
            // convective_heat_flux: None,
            // convert_volume_to_mass: false,
            // default: false,
            // dt_insert: 0.01_f64,
            // emissivity: 0.9_f64,
            // emissivity_back: None,
            // evac_default: false,
            // external_flux: None,
            // extinction_temperature: -273_f64,
            // fireline_mlr_max: f64,
            // free_slip: false,
            fyi: None,
            // geometry: "CARTESIAN".to_string(),
            // heat_of_vaporization: None,
            // heat_transfer_coefficient: None,
            // heat_transfer_coefficient_back: None,
            // heat_transfer_model: None,
            hrrpua: None,
            // ht3d: false,
            id: None,
            // ignition_temperature: 5000_f64,
            // inner_radius: None,
            // internal_heat_source: vec![],
            // layer_divide: None,
            // leak_path: vec![],
            // length: None,
            // mass_flux: None,
            // mass_flux_total: None,
            // mass_flux_var: None,
            // mass_fraction: vec![],
            // mass_transfer_coefficient: None,
            // matl_id: vec![],
            // matl_mass_fraction: vec![],
            // minimum_layer_thickness: 1e-6_f64,
            mlrpua: None,
            // , N_CELLS_MAX : String
            // n_layer_cells_max: vec![1000],
            // net_heat_flux: None,
            // no_slip: false,
            // nppc: 1_i64,
            // particle_mass_flux: None,
            // part_id: None,
            // ple: 0.3_f64,
            // profile: None,
            // radius: None,
            // ramp_ef: None,
            // ramp_mf: vec![],
            // ramp_part: None,
            // ramp_q: None,
            // ramp_t: None,
            // ramp_t_i: None,
            // ramp_v: None,
            // ramp_v_x: None,
            // ramp_v_y: None,
            // ramp_v_z: None,
            // rgb: RGB {r:255_i64, g: 204_i64, b: 102_i64},
            // roughness: 0_f64,
            // spec_id: None,
            // spread_rate: None,
            // stretch_factor: 2_f64,
            // tau_ef: 1_f64,
            // tau_mf: 1_f64,
            // tau_part: 1_f64,
            // tau_q: 1_f64,
            // tau_t: 1_f64,
            // tau_v: 1_f64,
            // texture_height: 1_f64,
            // texture_map: None,
            // texture_width: 1_f64,
            // tga_analysis: false,
            // tga_final_temperature: 800_f64,
            // tga_heating_rate: 5_f64,
            // thickness: vec![],
            // tmp_back: 20_f64,
            // tmp_front: 20_f64,
            // tmp_inner: vec![],
            // transparency: 1_f64,
            // vegetation: None,
            // vegetation_arrhenius_degrad: None,
            // vegetation_cdrag: None,
            // vegetation_char_fraction: None,
            // vegetation_element_density: None,
            // vegetation_ground_temp: None,
            // vegetation_height: None,
            // vegetation_initial_temp: None,
            // vegetation_layers: None,
            // vegetation_linear_degrad: None,
            // vegetation_load: None,
            // vegetation_lset_ignite_time: None,
            // veg_lset_qcon: None,
            // vegetation_moisture: None,
            // vegetation_no_burn: None,
            // vegetation_svratio: None,
            // veg_level_set_spread: None,
            // veg_lset_ros_back: None,
            // veg_lset_ros_flank: None,
            // veg_lset_ros_head: None,
            // veg_lset_wind_exp: None,
            // veg_lset_sigma: None,
            // veg_lset_ht: None,
            // veg_lset_beta: None,
            // veg_lset_ellipse: None,
            // veg_lset_tan2: None,
            // veg_lset_ellipse_head: None,
            // vel: None,
            // vel_bulk: None,
            // vel_grad: None,
            // vel_t: None,
            // volume_flow: None,
            // width: None,
            // xyz: None,
            // z0: 10_f64,
            // , ZETA_FRONT : String
            // , EXTERNAL_FLUX_RAMP : String
            // , TAU_EXTERNAL_FLUX : String
            // , VOLUME_FLUX : String
        }
    }
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
    fyi: Option<String>,
    id: String,
    table_data: Vec<f64>,
}

#[derive(Clone, Debug)]
pub struct Time {
    dt: Option<f64>,
    evac_dt_flowfield: f64,
    evac_dt_steady_state: f64,
    fyi: Option<String>,
    limiting_dt_ratio: f64,
    lock_time_step: bool,
    restrict_time_step: bool,
    t_begin: f64,
    t_end: f64,
    t_end_geom: f64,
    time_shrink_factor: f64,
    wall_increment: i64,
    wall_increment_ht3d: i64,
    twfin: f64,
}

#[derive(Clone, Debug)]
pub struct Trnx {
    cc: f64,
    fyi: Option<String>,
    ideriv: i64,
    mesh_number: i64,
    pc: f64,
}

#[derive(Clone, Debug)]
pub struct Trny {
    cc: f64,
    fyi: Option<String>,
    ideriv: i64,
    mesh_number: i64,
    pc: f64,
}

#[derive(Clone, Debug)]
pub struct Trnz {
    cc: f64,
    fyi: Option<String>,
    ideriv: i64,
    mesh_number: i64,
    pc: f64,
}

#[derive(Clone, Debug)]
pub struct Vent {
    color: Option<String>,
    ctrl_id: String,
    devc_id: String,
    dynamic_pressure: f64,
    evacuation: bool,
    fyi: Option<String>,
    id: Option<String>,
    ior: i64,
    l_eddy: f64,
    l_eddy_ij: Vec<i64>,
    mb: String,
    mesh_id: String,
    mult_id: String,
    n_eddy: i64,
    outline: bool,
    pbx: f64,
    pby: f64,
    pbz: f64,
    pressure_ramp: String,
    radius: f64,
    reynolds_stress: Vec<f64>,
    rgb: RGB,
    spread_rate: f64,
    surf_id: Option<String>,
    texture_origin: Vec<f64>,
    tmp_exterior: f64,
    tmp_exterior_ramp: String,
    transparency: f64,
    uvw: Vec<f64>,
    vel_rms: f64,
    // , WIND : String
    xb: Option<XB>,
    xyz: XYZ,
}

#[derive(Clone, Debug)]
pub struct Zone {
    pub id: String,
    pub leak_area: f64,
    pub leak_pressure_exponent: f64,
    pub leak_reference_pressure: f64,
    pub xb: XB,
    pub periodic: bool,
}

#[derive(Copy, Clone, Debug)]
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

#[derive(Copy, Clone, Debug)]
pub struct IJK {
    pub i: i64,
    pub j: i64,
    pub k: i64,
}

#[derive(Copy, Clone, Debug)]
pub struct RGB {
    pub r: i64,
    pub g: i64,
    pub b: i64,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct XB {
    pub x1: Coord,
    pub x2: Coord,
    pub y1: Coord,
    pub y2: Coord,
    pub z1: Coord,
    pub z2: Coord,
}

impl XB {
    /// Test if two XBs intersect (i.e. their bounding boxes). Two bounding boxes
    /// intersect of all 3 dimensions have overlap. EQ is considered overlap.
    pub fn intersect(&self, b: &XB) -> bool {
        let intersect_x = (self.x2 > b.x1) && (b.x2 > self.x1);
        let intersect_y = (self.y2 > b.y1) && (b.y2 > self.y1);
        let intersect_z = (self.z2 > b.z1) && (b.z2 > self.z1);
        intersect_x && intersect_y && intersect_z
    }
}



#[derive(Copy, Clone, Debug)]
pub struct XYZ {
    pub x: Coord,
    pub y: Coord,
    pub z: Coord,
}

type Coord = f64;

type GridCoord = i64;

fn simple_surf(name: String) -> Surf {
    Surf {
        id: Some(name),
        ..Default::default()
    }
}
/// Convert a ['NamelistFile'] to an ['FDSFile'].
pub fn decode_fds_file(namelist_file: &NamelistFile) -> FDSFile {
    let inert_surf = simple_surf("INERT".to_string());
    let open_surf = simple_surf("OPEN".to_string());
    let hvac_surf = simple_surf("HVAC".to_string());
    let mut fds_file = FDSFile {
        surfs: vec![inert_surf, open_surf, hvac_surf],
        ..Default::default()
    };
    for namelist in namelist_file.namelists.iter() {
        decode_namelist(&mut fds_file, &namelist);
    }
    fds_file
}

fn decode_namelist(fds_file: &mut FDSFile, namelist: &Namelist) {
    match namelist.name.as_ref() {
        "OBST" => decode_obst(fds_file, namelist),
        // "VENT" => decode_vent(fds_file, namelist),
        // "DEVC" => decode_devc(fds_file, namelist),
        // "PART" => decode_part(fds_file, namelist),
        // "TIME" => decode_time(fds_file, namelist),
        // "PROP" => decode_prop(fds_file, namelist),
        "SURF" => decode_surf(fds_file, namelist),
        "MESH" => decode_mesh(fds_file, namelist),
        // "SLCF" => decode_slcf(fds_file, namelist),
        // "REAC" => decode_reac(fds_file, namelist),
        // "HVAC" => decode_hvac(fds_file, namelist),
        // "DUMP" => decode_dump(fds_file, namelist),
        // "MISC" => decode_misc(fds_file, namelist),
        // "HEAD" => decode_head(fds_file, namelist),
        _ => decode_unknown(fds_file, namelist),
    }
}

fn decode_unknown(fds_file: &mut FDSFile, namelist: &Namelist) {
    fds_file.unknown_namelists.push(namelist.clone());
}

fn decode_obst(fds_file: &mut FDSFile, namelist: &Namelist) {
    let obst = Obst {
        //     allow_vent: bool,
        //     bndf_face: (bool, bool, bool, bool, bool, bool),
        //     bndf_obst: bool,
        //     bulk_density: Option<f64>,
        //     color: Option<String>,
        //     ctrl_id: Option<String>,
        //     devc_id: Option<String>,
        //     evacuation: bool,
        //     fyi: Option<String>,
        //     ht3d: bool,
        id: namelist.parameters.get("ID").map(|p| match &p.value {
            ParameterValue::Atom(ParameterValueAtom::String(s)) => s.clone(),
            ParameterValue::Atom(x) => panic!("Expected string atom, not {:?}", x),
            ParameterValue::Array(_) => panic!("Expected string atom, not array"),
        }),
        //     matl_id: Option<String>,
        //     mesh_id: Option<String>,
        //     mult_id: Option<String>,
        //     // , NOTERRAIN : bool
        //     outline: bool,
        //     overlay: bool,
        //     permit_hole: bool,
        //     prop_id: Option<String>,
        //     removable: bool,
        //     rgb: Option<RGB>,
        surf_id: namelist.parameters.get("SURF_ID").map(|p| match &p.value {
            ParameterValue::Atom(ParameterValueAtom::String(s)) => s.clone(),
            ParameterValue::Atom(x) => panic!("Expected string atom, not {:?}", x),
            ParameterValue::Array(_) => panic!("Expected string atom, not array"),
        }),
        // surf_id6: namelist.parameters.get("SURF_ID6").cloned(),
        // surf_ids: namelist.parameters.get("SURF_IDS").cloned(),
        //     texture_origin: XYZ,
        //     thicken: bool,
        //     transparency: f64,
        xb: {
            let v = namelist.parameters.get("XB").unwrap().clone();
            match v.value {
                ParameterValue::Atom(x) => panic!("Expected float array, not {:?}", x),
                ParameterValue::Array(array) => {
                    if array.values.len() != 6 {
                        panic!("Expected 6 values in array, found {}", array.values.len())
                    } else {
                        XB {
                            x1: match &array.values.get(&vec![1]).unwrap() {
                                ParameterValueAtom::Double(x) => x.clone(),
                                x => panic!("Expected string atom, not {:?}", x),
                            },
                            x2: match &array.values.get(&vec![2]).unwrap() {
                                ParameterValueAtom::Double(x) => x.clone(),
                                x => panic!("Expected string atom, not {:?}", x),
                            },
                            y1: match &array.values.get(&vec![3]).unwrap() {
                                ParameterValueAtom::Double(x) => x.clone(),
                                x => panic!("Expected string atom, not {:?}", x),
                            },
                            y2: match &array.values.get(&vec![4]).unwrap() {
                                ParameterValueAtom::Double(x) => x.clone(),
                                x => panic!("Expected string atom, not {:?}", x),
                            },
                            z1: match &array.values.get(&vec![5]).unwrap() {
                                ParameterValueAtom::Double(x) => x.clone(),
                                x => panic!("Expected string atom, not {:?}", x),
                            },
                            z2: match &array.values.get(&vec![6]).unwrap() {
                                ParameterValueAtom::Double(x) => x.clone(),
                                x => panic!("Expected string atom, not {:?}", x),
                            },
                        }
                    }
                }
            }
        },
    };
    fds_file.obsts.push(obst);
}

fn decode_devc(fds_file: &mut FDSFile, namelist: &Namelist) {
    let devc = Devc {
        prop_id: namelist.parameters.get("PROP_ID").map(|p| match &p.value {
            ParameterValue::Atom(ParameterValueAtom::String(s)) => s.clone(),
            ParameterValue::Atom(x) => panic!("Expected string atom, not {:?}", x),
            ParameterValue::Array(_) => panic!("Expected string atom, not array"),
        }),
        xyz: namelist.parameters.get("XYZ").map(|p| match &p.value {
                ParameterValue::Atom(x) => panic!("Expected float array, not {:?}", x),
                ParameterValue::Array(array) => {
                    if array.values.len() != 3 {
                        panic!("Expected 3 values in array, found {}", array.values.len())
                    } else {
                        XYZ {
                            x: match &array.values.get(&vec![1]).unwrap() {
                                ParameterValueAtom::Double(x) => x.clone(),
                                x => panic!("Expected string atom, not {:?}", x),
                            },
                            y: match &array.values.get(&vec![2]).unwrap() {
                                ParameterValueAtom::Double(x) => x.clone(),
                                x => panic!("Expected string atom, not {:?}", x),
                            },
                            z: match &array.values.get(&vec![3]).unwrap() {
                                ParameterValueAtom::Double(x) => x.clone(),
                                x => panic!("Expected string atom, not {:?}", x),
                            },
                        }
                    }
                }
            }),
    };
    fds_file.devcs.push(devc);
}


fn decode_surf(fds_file: &mut FDSFile, namelist: &Namelist) {
    let surf = Surf {
        adiabatic: match namelist.parameters.get("ADIABATIC") {
            None => false,
            Some(v) => match &v.value {
                ParameterValue::Atom(ParameterValueAtom::Bool(s)) => s.clone(),
                ParameterValue::Atom(x) => panic!("Expected double atom, not {:?}", x),
                ParameterValue::Array(_) => panic!("Expected double atom, not array"),
            }
        },
        auto_ignition_temperature: match namelist.parameters.get("AUTO_IGNITION_TEMPERATURE") {
            None => -273_f64,
            Some(v) => match &v.value {
                ParameterValue::Atom(ParameterValueAtom::Double(s)) => s.clone(),
                ParameterValue::Atom(x) => panic!("Expected double atom, not {:?}", x),
                ParameterValue::Array(_) => panic!("Expected double atom, not array"),
            }
        },
        color: namelist.parameters.get("COLOR").map(|p| match &p.value {
            ParameterValue::Atom(ParameterValueAtom::String(s)) => s.clone(),
            ParameterValue::Atom(x) => panic!("Expected string atom, not {:?}", x),
            ParameterValue::Array(_) => panic!("Expected string atom, not array"),
        }),
        fyi: namelist.parameters.get("FYI").map(|p| match &p.value {
            ParameterValue::Atom(ParameterValueAtom::String(s)) => s.clone(),
            ParameterValue::Atom(x) => panic!("Expected string atom, not {:?}", x),
            ParameterValue::Array(_) => panic!("Expected string atom, not array"),
        }),
        hrrpua: namelist.parameters.get("HRRPUA").map(|p| match &p.value {
            ParameterValue::Atom(ParameterValueAtom::Double(s)) => s.clone(),
            ParameterValue::Atom(x) => panic!("Expected double atom, not {:?}", x),
            ParameterValue::Array(_) => panic!("Expected double atom, not array"),
        }),
        id: namelist.parameters.get("ID").map(|p| match &p.value {
            ParameterValue::Atom(ParameterValueAtom::String(s)) => s.clone(),
            ParameterValue::Atom(x) => panic!("Expected string atom, not {:?}", x),
            ParameterValue::Array(_) => panic!("Expected string atom, not array"),
        }),
        mlrpua: namelist.parameters.get("MLRPUA").map(|p| match &p.value {
            ParameterValue::Atom(ParameterValueAtom::Double(s)) => s.clone(),
            ParameterValue::Atom(x) => panic!("Expected double atom, not {:?}", x),
            ParameterValue::Array(_) => panic!("Expected double atom, not array"),
        }),
    };
    fds_file.surfs.push(surf);
}


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
//             , STATISTICS_START = fromMaybe 0 $ parToDouble <$> getParameterMaybe nml "STATISTICS_START" -- todo: T_BEGIN is the default, how do we get that?
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

fn decode_mesh(fds_file: &mut FDSFile, namelist: &Namelist) {
    let mesh = Mesh {
        id: namelist.parameters.get("ID").map(|p| match &p.value {
            ParameterValue::Atom(ParameterValueAtom::String(s)) => s.clone(),
            ParameterValue::Atom(x) => panic!("Expected string atom, not {:?}", x),
            ParameterValue::Array(_) => panic!("Expected string atom, not array"),
        }),
        ijk: {
            let v = namelist.parameters.get("IJK").unwrap().clone();
            match v.value {
                ParameterValue::Atom(x) => panic!("Expected float array, not {:?}", x),
                ParameterValue::Array(array) => {
                    if array.values.len() != 3 {
                        panic!("Expected 3 values in array, found {}", array.values.len())
                    } else {
                        IJK {
                            i: match &array.values.get(&vec![1]).expect(&format!("no i value {:?}", array.values)) {
                                ParameterValueAtom::Int(x) => x.clone(),
                                x => panic!("Expected int atom, not {:?}", x),
                            },
                            j: match &array.values.get(&vec![2]).expect("no j value") {
                                ParameterValueAtom::Int(x) => x.clone(),
                                x => panic!("Expected int atom, not {:?}", x),
                            },
                            k: match &array.values.get(&vec![3]).expect("no k value") {
                                ParameterValueAtom::Int(x) => x.clone(),
                                x => panic!("Expected int atom, not {:?}", x),
                            },
                        }
                    }
                }
            }
        },
        xb: {
            let v = namelist.parameters.get("XB").unwrap().clone();
            match v.value {
                ParameterValue::Atom(x) => panic!("Expected float array, not {:?}", x),
                ParameterValue::Array(array) => {
                    if array.values.len() != 6 {
                        panic!("Expected 6 values in array, found {}", array.values.len())
                    } else {
                        XB {
                            x1: match &array.values.get(&vec![1]).expect("x1") {
                                ParameterValueAtom::Double(x) => x.clone(),
                                x => panic!("Expected string atom, not {:?}", x),
                            },
                            x2: match &array.values.get(&vec![2]).expect("x2") {
                                ParameterValueAtom::Double(x) => x.clone(),
                                x => panic!("Expected string atom, not {:?}", x),
                            },
                            y1: match &array.values.get(&vec![3]).expect("y1") {
                                ParameterValueAtom::Double(x) => x.clone(),
                                x => panic!("Expected string atom, not {:?}", x),
                            },
                            y2: match &array.values.get(&vec![4]).expect("y2") {
                                ParameterValueAtom::Double(x) => x.clone(),
                                x => panic!("Expected string atom, not {:?}", x),
                            },
                            z1: match &array.values.get(&vec![5]).expect("z1") {
                                ParameterValueAtom::Double(x) => x.clone(),
                                x => panic!("Expected string atom, not {:?}", x),
                            },
                            z2: match &array.values.get(&vec![6]).expect("z2") {
                                ParameterValueAtom::Double(x) => x.clone(),
                                x => panic!("Expected string atom, not {:?}", x),
                            },
                        }
                    }
                }
            }
        },
    };
    fds_file.meshes.push(mesh);
}

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
