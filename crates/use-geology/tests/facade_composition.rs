use use_geology::{
    fault, formation, fossil, geologic_process, geologic_time, mineral, rock, sediment, stratum,
    tectonic_plate,
};

#[test]
fn facade_composes_workspace_topics() -> Result<(), Box<dyn std::error::Error>> {
    let quartz = mineral::MineralName::new("Quartz")?;
    let hardness = mineral::MohsHardness::new(7.0)?;
    let basalt = rock::RockName::new("Basalt")?;
    let composition = rock::RockComposition::describe("mafic", ["Pyroxene", "Plagioclase"])?;
    let layer = stratum::StratumName::new("Brushy Basin")?;
    let thickness = stratum::StratumThickness::new(12.5)?;
    let formation_name = formation::FormationName::new("Morrison Formation")?;
    let fault_name = fault::FaultName::new("Wasatch Fault")?;
    let plate = tectonic_plate::TectonicPlateName::new("Pacific Plate")?;
    let epoch = geologic_time::GeologicEpoch::new("Holocene")?;
    let age = geologic_time::GeologicAge::new(145.0)?;
    let fossil_name = fossil::FossilName::new("Trilobite trace")?;
    let occurrence = fossil::FossilOccurrence::new(
        Some("Burgess Shale".to_string()),
        Some("Cambrian".to_string()),
    )?;
    let process = geologic_process::GeologicProcess::new("delta progradation")?;
    let rate = geologic_process::ProcessRate::new(0.2, "mm/yr")?;
    let sediment_name = sediment::SedimentName::new("Alluvial sand")?;
    let grain_size = sediment::GrainSize::new(0.25)?;

    assert_eq!(quartz.as_str(), "Quartz");
    assert_eq!(mineral::MineralClass::Silicate.to_string(), "silicate");
    assert_eq!(mineral::CrystalSystem::Trigonal.to_string(), "trigonal");
    assert!((hardness.value() - 7.0).abs() < f64::EPSILON);

    assert_eq!(basalt.as_str(), "Basalt");
    assert_eq!(rock::RockKind::Igneous.to_string(), "igneous");
    assert_eq!(rock::RockTexture::FineGrained.to_string(), "fine-grained");
    assert_eq!(composition.to_string(), "mafic [Pyroxene, Plagioclase]");

    assert_eq!(layer.as_str(), "Brushy Basin");
    assert_eq!(stratum::StratumKind::Layer.to_string(), "layer");
    assert_eq!(stratum::StratumOrder::new(3).position(), 3);
    assert!((thickness.meters() - 12.5).abs() < f64::EPSILON);

    assert_eq!(formation_name.as_str(), "Morrison Formation");
    assert_eq!(formation::FormationKind::Formation.to_string(), "formation");
    assert_eq!(
        formation::FormationMember::new("Brushy Basin Member")?.as_str(),
        "Brushy Basin Member"
    );
    assert_eq!(
        formation::FormationGroup::new("Chinle Group")?.as_str(),
        "Chinle Group"
    );

    assert_eq!(fault_name.as_str(), "Wasatch Fault");
    assert_eq!(fault::FaultKind::Normal.to_string(), "normal");
    assert_eq!(fault::FaultMovement::new("dip-slip")?.as_str(), "dip-slip");
    assert_eq!(fault::FaultActivity::Active.to_string(), "active");

    assert_eq!(plate.as_str(), "Pacific Plate");
    assert_eq!(tectonic_plate::PlateKind::Oceanic.to_string(), "oceanic");
    assert_eq!(
        tectonic_plate::PlateBoundaryKind::Divergent.to_string(),
        "divergent"
    );
    assert_eq!(
        tectonic_plate::PlateMotion::new("northwest")?.as_str(),
        "northwest"
    );

    assert_eq!(
        geologic_time::GeologicEon::Phanerozoic.to_string(),
        "phanerozoic"
    );
    assert_eq!(geologic_time::GeologicEra::Mesozoic.to_string(), "mesozoic");
    assert_eq!(
        geologic_time::GeologicPeriod::Jurassic.to_string(),
        "jurassic"
    );
    assert_eq!(epoch.as_str(), "Holocene");
    assert!((age.millions_of_years_before_present() - 145.0).abs() < f64::EPSILON);

    assert_eq!(fossil_name.as_str(), "Trilobite trace");
    assert_eq!(fossil::FossilKind::TraceFossil.to_string(), "trace-fossil");
    assert_eq!(
        fossil::FossilPreservation::Compressed.to_string(),
        "compressed"
    );
    assert_eq!(occurrence.to_string(), "Burgess Shale @ Cambrian");

    assert_eq!(process.as_str(), "delta progradation");
    assert_eq!(
        geologic_process::ProcessKind::Deposition.to_string(),
        "deposition"
    );
    assert_eq!(rate.to_string(), "0.2 mm/yr");

    assert_eq!(sediment_name.as_str(), "Alluvial sand");
    assert_eq!(sediment::SedimentKind::Sand.to_string(), "sand");
    assert!((grain_size.millimeters() - 0.25).abs() < f64::EPSILON);
    assert_eq!(sediment::Sorting::WellSorted.to_string(), "well-sorted");
    assert_eq!(sediment::Roundness::SubRounded.to_string(), "sub-rounded");

    Ok(())
}
