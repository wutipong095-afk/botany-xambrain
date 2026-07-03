/**
 * Botany-Xambrain food recommender — ดูสเปก wiki/food-recommender.md (U3)
 */
(function (global) {
  const PATIENT_CONTEXTS = [
    "ฟื้นตัว",
    "ท้องเสีย",
    "กลืนลำบาก",
    "คลื่นไส้อาเจียน",
    "หลังผ่าตัด",
    "แผลในปาก",
    "ผู้สูงอายุ",
    "หลังไข้",
  ];

  const THERMAL_RANK = { เย็น: 0, กลาง: 1, อุ่น: 2, ร้อน: 3 };

  function parseSymptoms(text) {
    return text
      .split(/[,，、\n]/)
      .map((s) => s.trim())
      .filter(Boolean);
  }

  function calcBmi(weightKg, heightCm) {
    const h = heightCm / 100;
    if (!weightKg || !h) return null;
    return weightKg / (h * h);
  }

  function bmiLabel(bmi) {
    if (bmi == null) return "—";
    if (bmi < 18.5) return "ผอม";
    if (bmi < 23) return "ปกติ";
    if (bmi < 25) return "น้ำหนักเกิน";
    return "อ้วน";
  }

  function allowedEnergy(bmi, patientMode) {
    if (patientMode) return ["ต่ำ", "กลาง"];
    if (bmi == null) return ["ต่ำ", "กลาง", "สูง"];
    if (bmi < 18.5) return ["ต่ำ", "กลาง", "สูง"];
    if (bmi < 25) return ["ต่ำ", "กลาง"];
    return ["ต่ำ"];
  }

  function matchSymptomRules(symptoms, rules) {
    const matched = [];
    for (const rule of rules) {
      const hit = symptoms.filter((s) =>
        rule.symptoms.some((r) => s.includes(r) || r.includes(s))
      );
      if (hit.length) matched.push({ rule, hit });
    }
    return matched;
  }

  function aggregateTargets(matchedRules) {
    const elements = new Set();
    const tastes = new Set();
    const thermals = new Set();
    const whys = [];

    for (const { rule, hit } of matchedRules) {
      elements.add(rule.element);
      rule.recommendTaste.forEach((t) => tastes.add(t));
      thermals.add(rule.recommendThermal);
      whys.push(`${hit.join(", ")} → ${rule.why}`);
    }

    return {
      elements: [...elements],
      tastes: [...tastes],
      thermals: [...thermals],
      whys,
    };
  }

  function thermalCompatible(menuThermal, targetThermals, patientMode) {
    if (!targetThermals.length) return true;
    const menuRank = THERMAL_RANK[menuThermal] ?? 1;
    if (patientMode) {
      return menuRank <= THERMAL_RANK.อุ่น;
    }
    return targetThermals.some((t) => {
      const targetRank = THERMAL_RANK[t] ?? 1;
      return Math.abs(menuRank - targetRank) <= 1;
    });
  }

  function scoreMenu(menu, ctx) {
    const {
      symptoms,
      targets,
      energyAllowed,
      patientMode,
      patientContexts,
    } = ctx;

    let score = 0;
    const reasons = [];

    if (!energyAllowed.includes(menu.energy)) {
      return { menu, score: -999, reasons: [`พลังงาน ${menu.energy} ไม่เข้าเกณฑ์ BMI`] };
    }

    if (patientMode) {
      if (!menu.patientFor?.length) {
        return { menu, score: -999, reasons: ["ไม่ใช่เมนูสำหรับผู้ป่วย"] };
      }
      const overlap = patientContexts.filter((p) => menu.patientFor.includes(p));
      if (patientContexts.length && !overlap.length) {
        return { menu, score: -999, reasons: ["ไม่ตรงบริบทผู้ป่วยที่เลือก"] };
      }
      score += overlap.length * 6;
      if (overlap.length) reasons.push(`บริบทผู้ป่วย: ${overlap.join(", ")}`);
      if (menu.tastes?.includes("เผ็ดร้อน")) {
        score -= 4;
        reasons.push("มีรสเผ็ดร้อน — ระวังในโหมดผู้ป่วย");
      }
      if (menu.tastes?.includes("จืด")) {
        score += 2;
      }
    }

    const symptomHits = symptoms.filter((s) =>
      menu.symptoms?.some((ms) => s.includes(ms) || ms.includes(s))
    );
    score += symptomHits.length * 4;
    if (symptomHits.length) reasons.push(`อาการตรง: ${symptomHits.join(", ")}`);

    for (const el of targets.elements) {
      if (menu.suitFor?.includes(el)) {
        score += 3;
        reasons.push(`เหมาะธาตุ${el}`);
      }
      if (menu.cautionFor?.includes(el)) {
        score -= 5;
        reasons.push(`ระวังธาตุ${el}`);
      }
    }

    for (const taste of targets.tastes) {
      if (menu.tastes?.includes(taste)) {
        score += 2;
      }
    }

    if (thermalCompatible(menu.thermal, targets.thermals, patientMode)) {
      score += 1;
    } else {
      score -= 2;
    }

    return { menu, score, reasons };
  }

  function recommend(input) {
    const {
      menus,
      rules,
      weightKg,
      heightCm,
      symptomText,
      patientMode = false,
      patientContexts = [],
      limit = 5,
    } = input;

    const symptoms = parseSymptoms(symptomText);
    const bmi = calcBmi(weightKg, heightCm);
    const energyAllowed = allowedEnergy(bmi, patientMode);
    const matchedRules = matchSymptomRules(symptoms, rules);
    const targets = aggregateTargets(matchedRules);

    const scored = menus
      .map((menu) =>
        scoreMenu(menu, {
          symptoms,
          targets,
          energyAllowed,
          patientMode,
          patientContexts,
        })
      )
      .filter((r) => r.score > -999)
      .sort((a, b) => b.score - a.score)
      .slice(0, limit);

    return {
      bmi,
      bmiLabel: bmiLabel(bmi),
      energyAllowed,
      matchedRules: matchedRules.map((m) => m.rule),
      targets,
      symptomWhys: targets.whys,
      results: scored,
      patientMode,
      patientContexts,
    };
  }

  global.FoodRecommender = {
    PATIENT_CONTEXTS,
    parseSymptoms,
    calcBmi,
    bmiLabel,
    recommend,
  };
})(typeof window !== "undefined" ? window : globalThis);
