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

  const COOK_METHOD_SCORE = {
    นึ่ง: 2,
    ย่าง: 1,
    อบ: 1,
    ต้ม: 1,
    ผัด: 0,
    ประกอบ: 0,
    ของว่าง: 0,
    อื่นๆ: 0,
    ทอด: -3,
  };

  function inferCookMethod(menu) {
    if (menu.cookMethod) return menu.cookMethod;
    const n = menu.name ?? "";
    if (/ทอด|ไข่เจียว/.test(n)) return "ทอด";
    if (/นึ่ง/.test(n)) return "นึ่ง";
    if (/ย่าง|เผา/.test(n)) return "ย่าง";
    if (/อบ/.test(n)) return "อบ";
    if (/ต้ม|โจ๊ก|ซุป|แกง|ข้าวต้ม|มันต้ม|ก๋วยเตี๋ยว/.test(n)) return "ต้ม";
    if (/ผัด/.test(n)) return "ผัด";
    if (/ยำ|ส้มตำ|ลาบ|น้ำตก|แจ่ว|เมี่ยง|น้ำพริก/.test(n)) return "ประกอบ";
    if (/น้ำ|ชา|เต้าฮวย|ไอศกรีม|ซาหริ่ม|ข้าวแช่|วุ้น|กล้วย|เชื่อม|ลอดช่อง|บัวลอย/.test(n)) {
      return "ของว่าง";
    }
    return "อื่นๆ";
  }

  function scoreCookMethod(menu, bmi, patientMode) {
    const method = inferCookMethod(menu);
    let delta = COOK_METHOD_SCORE[method] ?? 0;
    if (method === "ทอด" && bmi != null && bmi >= 25) delta -= 2;
    if (patientMode && (method === "นึ่ง" || method === "ต้ม" || method === "อบ")) delta += 1;
    return { method, delta };
  }

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
    const states = new Set();
    const stateByElement = {};
    const whys = [];

    for (const { rule, hit } of matchedRules) {
      elements.add(rule.element);
      rule.recommendTaste.forEach((t) => tastes.add(t));
      thermals.add(rule.recommendThermal);
      if (rule.state) {
        states.add(rule.state);
        stateByElement[rule.element] = rule.state;
      }
      whys.push(`${hit.join(", ")} → ${rule.why}`);
    }

    return {
      elements: [...elements],
      tastes: [...tastes],
      thermals: [...thermals],
      states: [...states],
      stateByElement,
      whys,
    };
  }

  function getAgeBand(age, bands) {
    if (age == null || Number.isNaN(age) || age < 0) return null;
    return bands.find((b) => age >= b.minAge && age <= b.maxAge) ?? null;
  }

  function mergeTargets(symptomTargets, ageTargets) {
    return {
      elements: [...new Set([...symptomTargets.elements, ...ageTargets.elements])],
      tastes: [...new Set([...symptomTargets.tastes, ...ageTargets.tastes])],
      thermals: [...new Set([...symptomTargets.thermals, ...ageTargets.thermals])],
      states: symptomTargets.states ?? [],
      stateByElement: symptomTargets.stateByElement ?? {},
      whys: [...symptomTargets.whys, ...ageTargets.whys],
      avoidTastes: ageTargets.avoidTastes ?? [],
      boostPatientContexts: ageTargets.boostPatientContexts ?? [],
      preferEnergy: ageTargets.preferEnergy ?? null,
    };
  }

  function aggregateAgeTargets(ageBand) {
    if (!ageBand) {
      return {
        elements: [],
        tastes: [],
        thermals: [],
        whys: [],
        avoidTastes: [],
        boostPatientContexts: [],
        preferEnergy: null,
      };
    }
    return {
      elements: [ageBand.dominantElement],
      tastes: ageBand.recommendTaste ?? [],
      thermals: [ageBand.recommendThermal],
      whys: [`อายุ ${ageBand.minAge}–${ageBand.maxAge} ปี (${ageBand.name}) · ${ageBand.samutthana} → ${ageBand.why}`],
      avoidTastes: ageBand.avoidTastes ?? [],
      boostPatientContexts: ageBand.boostPatientContexts ?? [],
      preferEnergy: ageBand.preferEnergy ?? null,
      band: ageBand,
    };
  }

  function refineEnergyAllowed(base, ageTargets, patientMode) {
    if (patientMode) return base;
    if (!ageTargets.preferEnergy?.length) return base;
    return base.filter((e) => ageTargets.preferEnergy.includes(e));
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
      ageBand,
      bmi,
    } = ctx;

    let score = 0;
    const reasons = [];

    if (!energyAllowed.includes(menu.energy)) {
      return { menu, score: -999, reasons: [`พลังงาน ${menu.energy} ไม่เข้าเกณฑ์`] };
    }

    if (targets.avoidTastes?.length) {
      const bad = targets.avoidTastes.filter((t) => menu.tastes?.includes(t));
      if (bad.length) {
        score -= bad.length * 4;
        reasons.push(`รสที่ควรหลีกเลี่ยงตามวัย: ${bad.join(", ")}`);
      }
    }

    if (ageBand) {
      reasons.push(`วัย ${ageBand.name} (สมุฏฐาน${ageBand.samutthana})`);
      const agePatientBoost = (targets.boostPatientContexts ?? []).filter((p) =>
        menu.patientFor?.includes(p)
      );
      score += agePatientBoost.length * 5;
      if (agePatientBoost.length) {
        reasons.push(`เหมาะวัยนี้: ${agePatientBoost.join(", ")}`);
      }
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

    if (targets.states?.length) {
      const hasDeficient = targets.states.includes("หย่อน");
      const hasExcess = targets.states.includes("กำเริบ");
      if (hasDeficient) {
        if (menu.energy === "สูง") {
          score += 2;
          reasons.push("มีอาการ 'หย่อน/พร่อง' → เสริมพลังงานสูง");
        } else if (menu.energy === "กลาง") {
          score += 1;
        }
      }
      if (hasExcess) {
        if (menu.energy === "ต่ำ") {
          score += 1;
          reasons.push("มีอาการ 'กำเริบ' → เมนูเบา/ระบาย");
        } else if (menu.energy === "สูง") {
          score -= patientMode ? 2 : 1;
          reasons.push("มีอาการ 'กำเริบ' → หลีกเลี่ยงเมนูหนัก");
        }
      }
    }

    if (thermalCompatible(menu.thermal, targets.thermals, patientMode)) {
      score += 1;
    } else {
      score -= 2;
    }

    const cook = scoreCookMethod(menu, bmi, patientMode);
    if (cook.delta) {
      score += cook.delta;
      if (cook.delta > 0) {
        reasons.push(`วิธีปรุง${cook.method} — ไขมันน้อยกว่าทอด`);
      } else if (cook.method === "ทอด") {
        reasons.push("วิธีปรุงทอด — ไขมันสูง ระวังน้ำหนักเกิน");
      }
    }

    return { menu, score, reasons };
  }

  function recommend(input) {
    const {
      menus,
      rules,
      ageBands = [],
      weightKg,
      heightCm,
      ageYears,
      symptomText,
      patientMode = false,
      patientContexts = [],
      limit = 5,
    } = input;

    const symptoms = parseSymptoms(symptomText);
    const bmi = calcBmi(weightKg, heightCm);
    const ageBand = getAgeBand(
      ageYears != null && ageYears !== "" ? Number(ageYears) : null,
      ageBands
    );
    let energyAllowed = allowedEnergy(bmi, patientMode);
    const matchedRules = matchSymptomRules(symptoms, rules);
    const symptomTargets = aggregateTargets(matchedRules);
    const ageTargets = aggregateAgeTargets(ageBand);
    const targets = mergeTargets(symptomTargets, ageTargets);
    energyAllowed = refineEnergyAllowed(energyAllowed, ageTargets, patientMode);

    const scored = menus
      .map((menu) =>
        scoreMenu(menu, {
          symptoms,
          targets,
          energyAllowed,
          patientMode,
          patientContexts,
          ageBand,
          bmi,
        })
      )
      .filter((r) => r.score > -999)
      .sort((a, b) => b.score - a.score)
      .slice(0, limit);

    return {
      bmi,
      bmiLabel: bmiLabel(bmi),
      ageYears: ageBand ? Number(ageYears) : null,
      ageBand,
      energyAllowed,
      matchedRules: matchedRules.map((m) => m.rule),
      targets,
      symptomWhys: targets.whys,
      results: scored,
      patientMode,
      patientContexts,
    };
  }

  function formatMenuAnalysis(menu) {
    if (!menu || menu.analysisTier < 2) return null;
    const ing = menu.ingredients || {};
    const wc = menu.whenCooked || {};
    return {
      tier: menu.analysisTier,
      core: ing.core || [],
      optional: ing.optional || [],
      layerS: menu.layerS || [],
      suitAudience: wc.suitAudience || [],
      avoidFor: wc.avoidFor || [],
      summary: wc.summary || menu.note || "",
      elements: menu.suitFor || [],
      tastes: menu.tastes || [],
      thermal: menu.thermal,
      energy: menu.energy,
    };
  }

  function escapeHtml(s) {
    return String(s)
      .replace(/&/g, "&amp;")
      .replace(/</g, "&lt;")
      .replace(/>/g, "&gt;")
      .replace(/"/g, "&quot;");
  }

  function renderMenuAnalysisHtml(menu) {
    const a = formatMenuAnalysis(menu);
    if (!a) return "";
    const list = (items) =>
      items.length
        ? `<ul class="analysis-list">${items.map((i) => `<li>${escapeHtml(i)}</li>`).join("")}</ul>`
        : "<p class=\"analysis-muted\">—</p>";
    return `
      <div class="analysis-detail" hidden>
        <p class="analysis-summary">${escapeHtml(a.summary)}</p>
        <div class="analysis-grid">
          <div>
            <strong>วัตถุดิบหลัก</strong>
            ${list(a.core)}
          </div>
          <div>
            <strong>เสริม (ตามฤดู)</strong>
            ${list(a.optional)}
          </div>
        </div>
        <p><strong>Layer S (ส่วนพืช)</strong></p>
        ${list(a.layerS)}
        <p><strong>เหมาะกับ (เมื่อปรุงสำเร็จ)</strong></p>
        ${list(a.suitAudience)}
        <p><strong>ควรระวัง</strong></p>
        ${list(a.avoidFor)}
        <p class="analysis-meta">ธาตุ ${escapeHtml(a.elements.join(", "))} · ฤทธิ์ ${escapeHtml(a.thermal)} · พลังงาน ${escapeHtml(a.energy)} · รส ${escapeHtml(a.tastes.join(" "))}</p>
      </div>`;
  }

  global.FoodRecommender = {
    PATIENT_CONTEXTS,
    parseSymptoms,
    calcBmi,
    bmiLabel,
    getAgeBand,
    inferCookMethod,
    formatMenuAnalysis,
    renderMenuAnalysisHtml,
    recommend,
  };
})(typeof window !== "undefined" ? window : globalThis);
