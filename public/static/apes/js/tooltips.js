const TOOLTIPS = {
    photosynthesis: "6CO₂ + 6H₂O → C₆H₁₂O₆ + 6O₂",
    respiration: "C₆H₁₂O₆ + 6O₂ → 6CO₂ + 6H₂O + energy (ATP)",
    sedimentation: "The process by which particles accumulate in layers and eventually form sedimentary rock.",
    combustion: "Hydrocarbon + O₂ → CO₂ + H₂O + energy (e.g., CH₄ + 2O₂ → CO₂ + 2H₂O + energy).",
    biosphere: "All living organisms and their ecosystems.",
    hydrosphere: "All the water on Earth- oceans, lakes, rivers, groundwater, and water vapor in the atmosphere.",
    lithosphere: "The crust and upper mantle (e.g., Rocks, minerals, landforms)."
};

document.activateTooltips = () => {
    var terms = document.querySelectorAll(".tooltip");
    terms.forEach((term) => {
        const key = term.textContent.toLowerCase().trim();
        const definition = TOOLTIPS[key];
        if (definition) {
            term.setAttribute("data-tooltip", definition)
        } else {
            term.setAttribute("data-tooltip", "An error was encountered when loading the definition. Please report this bug to dev@jayagra.com.")
        }
    })
};