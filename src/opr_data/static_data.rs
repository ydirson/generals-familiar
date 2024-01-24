// from https://army-forge.onepagerules.com/api/tts?id=...
pub const JSON_DATA: [&str; 7] = [
    // ybjR2-7kHUNY Prime Brothers
    r#"
{
   "campaignMode" : false,
   "competitive" : true,
   "gameSystem" : "gff",
   "id" : "ybjR2-7kHUNY",
   "isCloud" : false,
   "modified" : "2023-11-13T23:04:45.116Z",
   "name" : "Prime Brothers 200pts",
   "points" : 200,
   "pointsLimit" : 200,
   "selectedUnitId" : null,
   "specialRules" : [
      {
         "aliasedRuleId" : 48,
         "description" : "This model and all friendly units that are within 12” at the beginning of the round get AP(+1) when shooting. This effect lasts until the end of the round.",
         "hasRating" : false,
         "id" : 48,
         "name" : "Precision Shots"
      },
      {
         "aliasedRuleId" : 45,
         "description" : "This model and all friendly units that are within 12” at the beginning of the round get +1 to hit when shooting. This effect lasts until the end of the round.",
         "hasRating" : false,
         "id" : 45,
         "name" : "Battle Rites"
      },
      {
         "aliasedRuleId" : 857,
         "description" : "Attacks targeting units where all models have this rule count as having AP(-1), to a min. of AP(0).",
         "hasRating" : false,
         "id" : 857,
         "name" : "Heavy Shield"
      },
      {
         "aliasedRuleId" : null,
         "description" : "Once per activation, if within 2” of a model with Tough, roll one die. On a 2+ you may remove D3 wounds from that model.",
         "hasRating" : false,
         "id" : 126,
         "name" : "Repair"
      },
      {
         "aliasedRuleId" : null,
         "description" : "Gets +1 to hit in melee and shooting.",
         "hasRating" : false,
         "id" : 128,
         "name" : "Veteran Infantry"
      },
      {
         "aliasedRuleId" : 561,
         "description" : "This model and all friendly units that are within 12” at the beginning of the round get Regeneration. This effect lasts until the end of the round.",
         "hasRating" : false,
         "id" : 561,
         "name" : "Medical Training"
      },
      {
         "aliasedRuleId" : 55,
         "description" : "This model and all friendly units that are within 12” at the beginning of the round get Furious. If they already had Furious, they get extra hits on unmodified rolls of 5-6 instead. This effect lasts until the end of the round.",
         "hasRating" : false,
         "id" : 55,
         "name" : "War Chant"
      }
   ],
   "undoUnitRemove" : [
      {
         "armyId" : "oqnnu0gk8q6hyyny",
         "combined" : false,
         "cost" : 30,
         "defense" : 2,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "c3nZkAfH",
               "label" : "CCW (A2)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "83nhlNHF"
            },
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "wfEHdWKY",
               "label" : "Heavy Pistol (12\", A1, AP(1))",
               "name" : "Heavy Pistol",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "wfEHdWKY"
            }
         ],
         "id" : "hvNiOwU",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "c3nZkAfH",
               "label" : "CCW (A2)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "83nhlNHF"
            },
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "wfEHdWKY",
               "label" : "Heavy Pistol (12\", A1, AP(1))",
               "name" : "Heavy Pistol",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "wfEHdWKY"
            }
         ],
         "name" : "Assault Prime Brother",
         "notes" : null,
         "quality" : 3,
         "selectedUpgrades" : [],
         "selectionId" : "Xz6Ig",
         "size" : 1,
         "sortId" : 4,
         "specialRules" : [
            {
               "key" : "fearless",
               "name" : "Fearless",
               "rating" : ""
            }
         ],
         "traits" : [],
         "upgrades" : [
            "D1",
            "G1"
         ],
         "valid" : false,
         "xp" : 0
      }
   ],
   "unitPreview" : null,
   "units" : [
      {
         "armyId" : "oqnnu0gk8q6hyyny",
         "combined" : false,
         "cost" : 30,
         "defense" : 2,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "c3nZkAfH",
               "label" : "CCW (A2)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "83nhlNHF"
            },
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "wfEHdWKY",
               "label" : "Heavy Pistol (12\", A1, AP(1))",
               "name" : "Heavy Pistol",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "wfEHdWKY"
            }
         ],
         "id" : "hvNiOwU",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "c3nZkAfH",
               "label" : "CCW (A2)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "83nhlNHF"
            },
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "wfEHdWKY",
               "label" : "Heavy Pistol (12\", A1, AP(1))",
               "name" : "Heavy Pistol",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "wfEHdWKY"
            }
         ],
         "name" : "Assault Prime Brother",
         "notes" : null,
         "quality" : 3,
         "selectedUpgrades" : [],
         "selectionId" : "9mH34",
         "size" : 1,
         "specialRules" : [
            {
               "key" : "fearless",
               "name" : "Fearless",
               "rating" : ""
            }
         ],
         "traits" : [],
         "upgrades" : [
            "D1",
            "G1"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "oqnnu0gk8q6hyyny",
         "combined" : false,
         "cost" : 30,
         "defense" : 2,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "c3nZkAfH",
               "label" : "CCW (A2)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "83nhlNHF"
            },
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "wfEHdWKY",
               "label" : "Heavy Pistol (12\", A1, AP(1))",
               "name" : "Heavy Pistol",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "wfEHdWKY"
            }
         ],
         "id" : "hvNiOwU",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "c3nZkAfH",
               "label" : "CCW (A2)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "83nhlNHF"
            },
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "wfEHdWKY",
               "label" : "Heavy Pistol (12\", A1, AP(1))",
               "name" : "Heavy Pistol",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "wfEHdWKY"
            }
         ],
         "name" : "Assault Prime Brother",
         "notes" : null,
         "quality" : 3,
         "selectedUpgrades" : [],
         "selectionId" : "-38vE",
         "size" : 1,
         "specialRules" : [
            {
               "key" : "fearless",
               "name" : "Fearless",
               "rating" : ""
            }
         ],
         "traits" : [],
         "upgrades" : [
            "D1",
            "G1"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "oqnnu0gk8q6hyyny",
         "combined" : false,
         "cost" : 40,
         "defense" : 2,
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "8aOOK33l",
               "label" : "CCW (A1)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "4o5CBBZC"
            },
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "-_T9wTAv",
               "label" : "Flamer (12\", A1, Blast(3), Reliable)",
               "name" : "Flamer",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "blast",
                     "modify" : false,
                     "name" : "Blast",
                     "rating" : "3"
                  },
                  {
                     "key" : "reliable",
                     "modify" : false,
                     "name" : "Reliable",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "-_T9wTAv"
            }
         ],
         "id" : "NeuN0Gx",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "8aOOK33l",
               "label" : "CCW (A1)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "4o5CBBZC"
            },
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "-_T9wTAv",
               "label" : "Flamer (12\", A1, Blast(3), Reliable)",
               "name" : "Flamer",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "blast",
                     "modify" : false,
                     "name" : "Blast",
                     "rating" : "3"
                  },
                  {
                     "key" : "reliable",
                     "modify" : false,
                     "name" : "Reliable",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "-_T9wTAv"
            }
         ],
         "name" : "Infernal",
         "notes" : null,
         "quality" : 3,
         "selectedUpgrades" : [],
         "selectionId" : "wSM6O",
         "size" : 1,
         "specialRules" : [
            {
               "id" : 65,
               "key" : "fearless",
               "name" : "Fearless"
            },
            {
               "id" : 74,
               "key" : "relentless",
               "name" : "Relentless"
            }
         ],
         "traits" : [],
         "upgrades" : [],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "oqnnu0gk8q6hyyny",
         "combined" : false,
         "cost" : 60,
         "defense" : 3,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "c3nZkAfH",
               "label" : "CCW (A2)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "83nhlNHF"
            },
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "wfEHdWKY",
               "label" : "Master Heavy Pistol (12\", A2, AP(1))",
               "name" : "Master Heavy Pistol",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "0EpTPX9H"
            }
         ],
         "id" : "qGGJpGt",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "c3nZkAfH",
               "label" : "CCW (A2)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "83nhlNHF"
            },
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "wfEHdWKY",
               "label" : "Master Heavy Pistol (12\", A2, AP(1))",
               "name" : "Master Heavy Pistol",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "0EpTPX9H"
            }
         ],
         "name" : "Elite Raider",
         "notes" : null,
         "quality" : 4,
         "selectedUpgrades" : [],
         "selectionId" : "ui7dg",
         "size" : 1,
         "specialRules" : [
            {
               "key" : "fearless",
               "name" : "Fearless",
               "rating" : ""
            },
            {
               "additional" : false,
               "key" : "furious",
               "name" : "Furious",
               "rating" : ""
            },
            {
               "key" : "hero",
               "name" : "Hero",
               "rating" : ""
            },
            {
               "key" : "strider",
               "name" : "Strider",
               "rating" : ""
            },
            {
               "key" : "tough",
               "name" : "Tough",
               "rating" : "3"
            }
         ],
         "traits" : [],
         "upgrades" : [
            "C1",
            "qIWMF",
            "A1"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "oqnnu0gk8q6hyyny",
         "combined" : false,
         "cost" : 40,
         "defense" : 2,
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "8aOOK33l",
               "label" : "CCW (A1)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "4o5CBBZC"
            },
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "-_T9wTAv",
               "label" : "Flamer (12\", A1, Blast(3), Reliable)",
               "name" : "Flamer",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "blast",
                     "modify" : false,
                     "name" : "Blast",
                     "rating" : "3"
                  },
                  {
                     "key" : "reliable",
                     "modify" : false,
                     "name" : "Reliable",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "-_T9wTAv"
            }
         ],
         "id" : "NeuN0Gx",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "8aOOK33l",
               "label" : "CCW (A1)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "4o5CBBZC"
            },
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "-_T9wTAv",
               "label" : "Flamer (12\", A1, Blast(3), Reliable)",
               "name" : "Flamer",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "blast",
                     "modify" : false,
                     "name" : "Blast",
                     "rating" : "3"
                  },
                  {
                     "key" : "reliable",
                     "modify" : false,
                     "name" : "Reliable",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "-_T9wTAv"
            }
         ],
         "name" : "Infernal",
         "notes" : null,
         "quality" : 3,
         "selectedUpgrades" : [],
         "selectionId" : "4AxnW",
         "size" : 1,
         "specialRules" : [
            {
               "id" : 65,
               "key" : "fearless",
               "name" : "Fearless"
            },
            {
               "id" : 74,
               "key" : "relentless",
               "name" : "Relentless"
            }
         ],
         "traits" : [],
         "upgrades" : [],
         "valid" : true,
         "xp" : 0
      }
   ]
}
"#,
    // dVlqH2ICxln2 Robot Legions
    r#"
{
   "campaignMode" : false,
   "competitive" : true,
   "gameSystem" : "gff",
   "id" : "dVlqH2ICxln2",
   "isCloud" : false,
   "modified" : "2023-11-13T23:08:46.726Z",
   "name" : "Robot Legions 200pts",
   "points" : 200,
   "pointsLimit" : 200,
   "selectedUnitId" : null,
   "specialRules" : [
      {
         "aliasedRuleId" : null,
         "description" : "Once per game, when this model is activated, you may place a new unit of 3 Bot Swarms fully within 6” of it.",
         "hasRating" : false,
         "id" : 266,
         "name" : "Release Swarm"
      },
      {
         "aliasedRuleId" : 162,
         "description" : "This model and all friendly units that are within 12” at the beginning of the round may ignore the Slow rule. This effect lasts until the end of the round.",
         "hasRating" : false,
         "id" : 162,
         "name" : "Royal March"
      },
      {
         "aliasedRuleId" : 51,
         "description" : "This model and up to half of its army get Ambush (must deploy within 3” of this model).",
         "hasRating" : false,
         "id" : 51,
         "name" : "Shadow-Protocol"
      },
      {
         "aliasedRuleId" : null,
         "description" : "This model and all friendly units that are within 12” at the beginning of the round get +1 to Regeneration rolls. This effect lasts until the end of the round.",
         "hasRating" : false,
         "id" : 265,
         "name" : "Regen-Protocol"
      },
      {
         "aliasedRuleId" : null,
         "description" : "Once per activation, pick one friendly unit within 12”, which gets +1 to Regeneration rolls next time it takes wounds.",
         "hasRating" : false,
         "id" : 264,
         "name" : "Reanimator"
      },
      {
         "aliasedRuleId" : null,
         "description" : "Once per activation, pick one friendly Caster within 6”, which gets +1 to its next spell casting roll.",
         "hasRating" : false,
         "id" : 263,
         "name" : "Spell Warden"
      },
      {
         "aliasedRuleId" : null,
         "description" : "Once per activation, pick one friendly unit within 12”, which may ignore the Slow rule next time it moves.",
         "hasRating" : false,
         "id" : 269,
         "name" : "Royal March Order"
      },
      {
         "aliasedRuleId" : null,
         "description" : "This model counts as having Caster(6), but takes 3 wounds whenever it fails to cast a spell.",
         "hasRating" : false,
         "id" : 262,
         "name" : "Spell Master"
      },
      {
         "aliasedRuleId" : null,
         "description" : "Counts as having the Ambush rule, and gets AP(+3) when shooting on the round in which it deploys.",
         "hasRating" : false,
         "id" : 261,
         "name" : "Hunter"
      },
      {
         "aliasedRuleId" : null,
         "description" : "This model gets +1 to defense rolls against non-spell attacks.",
         "hasRating" : false,
         "id" : 858,
         "name" : "Shield Wall"
      },
      {
         "aliasedRuleId" : null,
         "description" : "Once per activation, if within 2” of a model with Tough, roll one die. On a 2+ you may remove D3 wounds from that model.",
         "hasRating" : false,
         "id" : 126,
         "name" : "Repair"
      },
      {
         "aliasedRuleId" : null,
         "description" : "Enemy units can’t be set up within 12” of this model when using Ambush.",
         "hasRating" : false,
         "id" : 370,
         "name" : "Warning Cry"
      },
      {
         "aliasedRuleId" : null,
         "description" : "This model counts as having Ambush, and may be deployed up to 1” away from enemy units.",
         "hasRating" : false,
         "id" : 166,
         "name" : "Tunneller"
      },
      {
         "aliasedRuleId" : 149,
         "description" : "When this model and all friendly units that are within 12” at the beginning of the round take a wound, roll one die, and on a 6+ it is ignored. If the wound was from a spell, then it is ignored on a 4+ instead. This effect lasts until the end of the round.",
         "hasRating" : false,
         "id" : 149,
         "name" : "Gloom-Protocol"
      },
      {
         "aliasedRuleId" : 216,
         "description" : "Whenever this unit fails a morale test, it counts as passed instead. Then, roll as many dice as remaining models/tough with this rule, and for each result of 1-3 the unit takes one wound, which can’t be ignored.",
         "hasRating" : false,
         "id" : 216,
         "name" : "Robot"
      },
      {
         "aliasedRuleId" : null,
         "description" : "Unmodified rolls of 6 are multiplied by 2 (only the original hit counts as a 6).",
         "hasRating" : false,
         "id" : 846,
         "name" : "Flux"
      }
   ],
   "undoUnitRemove" : [
      {
         "armyId" : "4k5amkxoybdiqotm",
         "combined" : false,
         "cost" : 25,
         "defense" : 6,
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 3,
               "count" : 1,
               "id" : "DvYpHLlf",
               "label" : "Swarm Attack (A3, Rending)",
               "name" : "Swarm Attack",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "bUQRotbI"
            }
         ],
         "id" : "yoeQi4m",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 3,
               "count" : 1,
               "id" : "DvYpHLlf",
               "label" : "Swarm Attack (A3, Rending)",
               "name" : "Swarm Attack",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "bUQRotbI"
            }
         ],
         "name" : "Bot Swarm",
         "notes" : null,
         "quality" : 6,
         "selectedUpgrades" : [],
         "selectionId" : "7vmue",
         "size" : 1,
         "sortId" : 1,
         "specialRules" : [
            {
               "key" : "regeneration",
               "name" : "Regeneration",
               "rating" : ""
            },
            {
               "key" : "robot",
               "name" : "Robot",
               "rating" : ""
            },
            {
               "key" : "strider",
               "name" : "Strider",
               "rating" : ""
            },
            {
               "key" : "tough",
               "name" : "Tough",
               "rating" : "3"
            }
         ],
         "traits" : [],
         "upgrades" : [],
         "valid" : true,
         "xp" : 0
      }
   ],
   "unitPreview" : null,
   "units" : [
      {
         "armyId" : "4k5amkxoybdiqotm",
         "combined" : false,
         "cost" : 65,
         "defense" : 3,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 3,
               "count" : 1,
               "id" : "0L3LR8_n",
               "label" : "CCW (A3)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "tO5jEqMO"
            },
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "_g6MjuFl",
               "label" : "Lord Gauss Pistol (12\", A2, Rending)",
               "name" : "Lord Gauss Pistol",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "_g6MjuFl"
            }
         ],
         "id" : "lcEWPMS",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 3,
               "count" : 1,
               "id" : "0L3LR8_n",
               "label" : "CCW (A3)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "tO5jEqMO"
            },
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "_g6MjuFl",
               "label" : "Lord Gauss Pistol (12\", A2, Rending)",
               "name" : "Lord Gauss Pistol",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "_g6MjuFl"
            }
         ],
         "name" : "Robot Lord",
         "notes" : null,
         "quality" : 3,
         "selectedUpgrades" : [],
         "selectionId" : "0q_KU",
         "size" : 1,
         "specialRules" : [
            {
               "key" : "hero",
               "name" : "Hero",
               "rating" : ""
            },
            {
               "key" : "regeneration",
               "name" : "Regeneration",
               "rating" : ""
            },
            {
               "key" : "robot",
               "name" : "Robot",
               "rating" : ""
            },
            {
               "key" : "slow",
               "name" : "Slow",
               "rating" : ""
            },
            {
               "key" : "tough",
               "name" : "Tough",
               "rating" : "3"
            }
         ],
         "traits" : [],
         "upgrades" : [
            "A1",
            "B1",
            "C1"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "4k5amkxoybdiqotm",
         "combined" : false,
         "cost" : 25,
         "defense" : 4,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "_WpwGRUk",
               "label" : "CCW (A1)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "4o5CBBZC"
            },
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "OaYz4pdg",
               "label" : "Gauss Rifle (24\", A1, Rending)",
               "name" : "Gauss Rifle",
               "originalCount" : 1,
               "range" : 24,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "OaYz4pdg"
            }
         ],
         "id" : "njiIkJi",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "_WpwGRUk",
               "label" : "CCW (A1)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "4o5CBBZC"
            },
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "OaYz4pdg",
               "label" : "Gauss Rifle (24\", A1, Rending)",
               "name" : "Gauss Rifle",
               "originalCount" : 1,
               "range" : 24,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "OaYz4pdg"
            }
         ],
         "name" : "Warrior",
         "notes" : null,
         "quality" : 3,
         "selectedUpgrades" : [],
         "selectionId" : "vGlOj",
         "size" : 1,
         "specialRules" : [
            {
               "key" : "regeneration",
               "name" : "Regeneration",
               "rating" : ""
            },
            {
               "key" : "robot",
               "name" : "Robot",
               "rating" : ""
            },
            {
               "key" : "slow",
               "name" : "Slow",
               "rating" : ""
            }
         ],
         "traits" : [],
         "upgrades" : [
            "E1"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "4k5amkxoybdiqotm",
         "combined" : false,
         "cost" : 25,
         "defense" : 4,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "_WpwGRUk",
               "label" : "CCW (A1)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "4o5CBBZC"
            },
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "OaYz4pdg",
               "label" : "Gauss Rifle (24\", A1, Rending)",
               "name" : "Gauss Rifle",
               "originalCount" : 1,
               "range" : 24,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "OaYz4pdg"
            }
         ],
         "id" : "njiIkJi",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "_WpwGRUk",
               "label" : "CCW (A1)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "4o5CBBZC"
            },
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "OaYz4pdg",
               "label" : "Gauss Rifle (24\", A1, Rending)",
               "name" : "Gauss Rifle",
               "originalCount" : 1,
               "range" : 24,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "OaYz4pdg"
            }
         ],
         "name" : "Warrior",
         "notes" : null,
         "quality" : 3,
         "selectedUpgrades" : [],
         "selectionId" : "V8KE9",
         "size" : 1,
         "specialRules" : [
            {
               "key" : "regeneration",
               "name" : "Regeneration",
               "rating" : ""
            },
            {
               "key" : "robot",
               "name" : "Robot",
               "rating" : ""
            },
            {
               "key" : "slow",
               "name" : "Slow",
               "rating" : ""
            }
         ],
         "traits" : [],
         "upgrades" : [
            "E1"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "4k5amkxoybdiqotm",
         "combined" : false,
         "cost" : 30,
         "defense" : 3,
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "Q47Qlon8",
               "label" : "Metal Claws (A2, AP(1))",
               "name" : "Metal Claws",
               "originalCount" : 1,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "Q47Qlon8"
            }
         ],
         "id" : "6q8HvOZ",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "Q47Qlon8",
               "label" : "Metal Claws (A2, AP(1))",
               "name" : "Metal Claws",
               "originalCount" : 1,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "Q47Qlon8"
            }
         ],
         "name" : "Flesh-Eater",
         "notes" : null,
         "quality" : 3,
         "selectedUpgrades" : [],
         "selectionId" : "aWCA1",
         "size" : 1,
         "specialRules" : [
            {
               "key" : "regeneration",
               "name" : "Regeneration",
               "rating" : ""
            },
            {
               "key" : "robot",
               "name" : "Robot",
               "rating" : ""
            },
            {
               "key" : "slow",
               "name" : "Slow",
               "rating" : ""
            },
            {
               "key" : "tunneller",
               "name" : "Tunneller",
               "rating" : ""
            }
         ],
         "traits" : [],
         "upgrades" : [],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "4k5amkxoybdiqotm",
         "combined" : false,
         "cost" : 30,
         "defense" : 3,
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "Q47Qlon8",
               "label" : "Metal Claws (A2, AP(1))",
               "name" : "Metal Claws",
               "originalCount" : 1,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "Q47Qlon8"
            }
         ],
         "id" : "6q8HvOZ",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "Q47Qlon8",
               "label" : "Metal Claws (A2, AP(1))",
               "name" : "Metal Claws",
               "originalCount" : 1,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "Q47Qlon8"
            }
         ],
         "name" : "Flesh-Eater",
         "notes" : null,
         "quality" : 3,
         "selectedUpgrades" : [],
         "selectionId" : "Fhi38",
         "size" : 1,
         "specialRules" : [
            {
               "key" : "regeneration",
               "name" : "Regeneration",
               "rating" : ""
            },
            {
               "key" : "robot",
               "name" : "Robot",
               "rating" : ""
            },
            {
               "key" : "slow",
               "name" : "Slow",
               "rating" : ""
            },
            {
               "key" : "tunneller",
               "name" : "Tunneller",
               "rating" : ""
            }
         ],
         "traits" : [],
         "upgrades" : [],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "4k5amkxoybdiqotm",
         "combined" : false,
         "cost" : 25,
         "defense" : 6,
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 3,
               "count" : 1,
               "id" : "DvYpHLlf",
               "label" : "Swarm Attack (A3, Rending)",
               "name" : "Swarm Attack",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "bUQRotbI"
            }
         ],
         "id" : "yoeQi4m",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 3,
               "count" : 1,
               "id" : "DvYpHLlf",
               "label" : "Swarm Attack (A3, Rending)",
               "name" : "Swarm Attack",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "bUQRotbI"
            }
         ],
         "name" : "Bot Swarm",
         "notes" : null,
         "quality" : 6,
         "selectedUpgrades" : [],
         "selectionId" : "Q-DFh",
         "size" : 1,
         "specialRules" : [
            {
               "key" : "regeneration",
               "name" : "Regeneration",
               "rating" : ""
            },
            {
               "key" : "robot",
               "name" : "Robot",
               "rating" : ""
            },
            {
               "key" : "strider",
               "name" : "Strider",
               "rating" : ""
            },
            {
               "key" : "tough",
               "name" : "Tough",
               "rating" : "3"
            }
         ],
         "traits" : [],
         "upgrades" : [],
         "valid" : true,
         "xp" : 0
      }
   ]
}
"#,
    // nzTpaov-wlwd Blessed Sisters
        r#"
{
   "campaignMode" : false,
   "competitive" : true,
   "gameSystem" : "gff",
   "id" : "nzTpaov-wlwd",
   "isCloud" : false,
   "modified" : "2023-11-15T10:30:10.932Z",
   "name" : "Blessed Sisters 200pts v0",
   "points" : 200,
   "pointsLimit" : 200,
   "selectedUnitId" : "uAmW_",
   "specialRules" : [
      {
         "aliasedRuleId" : null,
         "description" : "When shooting at enemies within 12\", hits from unmodified rolls of 5-6 are multiplied by 2 (only the original hit counts as a 6).",
         "hasRating" : false,
         "id" : 991,
         "name" : "Highly Devout"
      },
      {
         "aliasedRuleId" : 48,
         "description" : "This model and all friendly units that are within 12” at the beginning of the round get AP(+1) when shooting. This effect lasts until the end of the round.",
         "hasRating" : false,
         "id" : 48,
         "name" : "Spiritual Guidance"
      },
      {
         "aliasedRuleId" : 50,
         "description" : "This model and all friendly units that are within 12” at the beginning of the round get Stealth. This effect lasts until the end of the round.",
         "hasRating" : false,
         "id" : 50,
         "name" : "Blind Faith"
      },
      {
         "aliasedRuleId" : 47,
         "description" : "This model and all friendly units that are within 12” at the beginning of the round get AP(+1) in melee. This effect lasts until the end of the round.",
         "hasRating" : false,
         "id" : 47,
         "name" : "War Hymns"
      },
      {
         "aliasedRuleId" : 53,
         "description" : "This model and all friendly units that are within 12” at the beginning of the round get +1 to morale test rolls. This effect lasts until the end of the round.",
         "hasRating" : false,
         "id" : 53,
         "name" : "Canticle Megaphone"
      },
      {
         "aliasedRuleId" : null,
         "description" : "Counts as having Furious and gets AP(+1) when charging.",
         "hasRating" : false,
         "id" : 212,
         "name" : "Frenzy"
      },
      {
         "aliasedRuleId" : null,
         "description" : "This model gets +1 to defense rolls against non-spell attacks.",
         "hasRating" : false,
         "id" : 858,
         "name" : "Shield Wall"
      },
      {
         "aliasedRuleId" : null,
         "description" : "When shooting at enemies within 12\", hits from unmodified rolls of 6 are multiplied by 2 (only the original hit counts as a 6).",
         "hasRating" : null,
         "id" : 895,
         "name" : "Devout"
      },
      {
         "aliasedRuleId" : null,
         "description" : "Once per activation, pick 2 friendly units within 6”. Those units, and all friendly units within 6\" get +1 to defense rolls next time they take hits.",
         "hasRating" : false,
         "id" : 417,
         "name" : "Protective Aura"
      },
      {
         "aliasedRuleId" : 128,
         "description" : "Gets +1 to hit in melee and shooting.",
         "hasRating" : false,
         "id" : 128,
         "name" : "Celestial Infantry"
      },
      {
         "aliasedRuleId" : 422,
         "description" : "Once per activation, pick 2 friendly units within 6”. Those units, and all friendly units within 6\" get +1 to hit next time they fight in melee.",
         "hasRating" : false,
         "id" : 422,
         "name" : "Holy Statue"
      },
      {
         "aliasedRuleId" : 224,
         "description" : "Once per activation, pick 2 friendly units within 12”. Those units, and all friendly units within 6\" get +1 to their next morale test roll.",
         "hasRating" : false,
         "id" : 224,
         "name" : "Angelic Aura"
      },
      {
         "aliasedRuleId" : 561,
         "description" : "This model and all friendly units that are within 12” at the beginning of the round get Regeneration. This effect lasts until the end of the round.",
         "hasRating" : false,
         "id" : 561,
         "name" : "Medical Training"
      }
   ],
   "undoUnitRemove" : [
      {
         "armyId" : "7oi8zeiqfamiur21",
         "combined" : false,
         "cost" : 25,
         "defense" : 4,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "WWqUi2Wx",
               "label" : "CCW (A1)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "4o5CBBZC"
            },
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "GKm7CCrS",
               "label" : "Twin Heavy Pistol (12\", A2, AP(1))",
               "name" : "Twin Heavy Pistol",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "GKm7CCrS"
            }
         ],
         "id" : "qgESKb_",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "WWqUi2Wx",
               "label" : "CCW (A1)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "4o5CBBZC"
            },
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "GKm7CCrS",
               "label" : "Twin Heavy Pistol (12\", A2, AP(1))",
               "name" : "Twin Heavy Pistol",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "GKm7CCrS"
            }
         ],
         "name" : "Pistoleer Sister",
         "notes" : null,
         "quality" : 4,
         "selectedUpgrades" : [],
         "selectionId" : "CiQIu",
         "size" : 1,
         "sortId" : 5,
         "specialRules" : [
            {
               "key" : "ambush",
               "name" : "Ambush",
               "rating" : ""
            },
            {
               "additional" : false,
               "key" : "devout",
               "name" : "Devout",
               "rating" : ""
            },
            {
               "key" : "flying",
               "name" : "Flying",
               "rating" : ""
            }
         ],
         "traits" : [],
         "upgrades" : [
            "J1"
         ],
         "valid" : true,
         "xp" : 0
      }
   ],
   "unitPreview" : null,
   "units" : [
      {
         "armyId" : "7oi8zeiqfamiur21",
         "combined" : false,
         "cost" : 45,
         "defense" : 4,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "WWqUi2Wx",
               "label" : "CCW (A2)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "83nhlNHF"
            },
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "lXOrCYv-",
               "label" : "Master Heavy Pistol (12\", A2, AP(1))",
               "name" : "Master Heavy Pistol",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "r63zpLTC"
            }
         ],
         "id" : "2TXWi3B",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "WWqUi2Wx",
               "label" : "CCW (A2)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "83nhlNHF"
            },
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "lXOrCYv-",
               "label" : "Master Heavy Pistol (12\", A2, AP(1))",
               "name" : "Master Heavy Pistol",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "r63zpLTC"
            },
            {
               "content" : [
                  {
                     "count" : 1,
                     "dependencies" : [],
                     "key" : "blind faith",
                     "label" : "Blind Faith",
                     "modify" : false,
                     "name" : "Blind Faith",
                     "rating" : "",
                     "type" : "ArmyBookRule"
                  }
               ],
               "count" : 1,
               "dependencies" : [],
               "label" : "Superior (Blind Faith)",
               "name" : "Superior",
               "type" : "ArmyBookItem"
            }
         ],
         "name" : "High Sister",
         "notes" : null,
         "quality" : 4,
         "selectedUpgrades" : [
            {
               "instanceId" : "anJyCPdDH",
               "option" : {
                  "cost" : 25,
                  "costs" : [
                     {
                        "cost" : 25,
                        "unitId" : "2TXWi3B"
                     },
                     {
                        "cost" : 25,
                        "unitId" : "_nVdDbE"
                     }
                  ],
                  "gains" : [
                     {
                        "content" : [
                           {
                              "count" : 1,
                              "dependencies" : [],
                              "key" : "blind faith",
                              "label" : "Blind Faith",
                              "modify" : false,
                              "name" : "Blind Faith",
                              "rating" : "",
                              "type" : "ArmyBookRule"
                           }
                        ],
                        "count" : 1,
                        "dependencies" : [],
                        "label" : "Superior (Blind Faith)",
                        "name" : "Superior",
                        "type" : "ArmyBookItem"
                     }
                  ],
                  "id" : "IJLtqfK",
                  "label" : "Superior (Blind Faith)",
                  "parentPackageUid" : "B1",
                  "parentSectionUid" : "jDPiWW0",
                  "proposedCost" : 25,
                  "proposedCostHint" : [
                     {
                        "isValid" : true,
                        "newCostPrecise" : "23.8",
                        "newCostRounded" : 25,
                        "unitName" : "High Sister"
                     },
                     {
                        "isValid" : true,
                        "newCostPrecise" : "23.8",
                        "newCostRounded" : 25,
                        "unitName" : "Novice Leader"
                     }
                  ],
                  "proposedVersion" : "2.50",
                  "type" : "ArmyBookUpgradeOption",
                  "uid" : "IJLtqfK"
               },
               "upgrade" : {
                  "id" : "3xgyu6N",
                  "isHeroUpgrade" : true,
                  "label" : "Upgrade with one",
                  "options" : null,
                  "parentPackageUid" : "B1",
                  "select" : {
                     "type" : "exactly",
                     "value" : 1
                  },
                  "type" : "ArmyBookUpgradeSection",
                  "uid" : "jDPiWW0",
                  "variant" : "upgrade"
               }
            }
         ],
         "selectionId" : "fBaU-",
         "size" : 1,
         "specialRules" : [
            {
               "additional" : false,
               "key" : "devout",
               "name" : "Devout",
               "rating" : ""
            },
            {
               "key" : "hero",
               "name" : "Hero",
               "rating" : ""
            },
            {
               "key" : "tough",
               "name" : "Tough",
               "rating" : "3"
            }
         ],
         "traits" : [],
         "upgrades" : [
            "A1",
            "B1",
            "C1"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "7oi8zeiqfamiur21",
         "combined" : false,
         "cost" : 15,
         "defense" : 5,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : "2",
               "count" : 1,
               "id" : "4MIc_Zub",
               "label" : "Chainsaw Sword (A2, AP(1))",
               "name" : "Chainsaw Sword",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "CClql4IC"
            }
         ],
         "id" : "R3sC7UX",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : "2",
               "count" : 1,
               "id" : "4MIc_Zub",
               "label" : "Chainsaw Sword (A2, AP(1))",
               "name" : "Chainsaw Sword",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "CClql4IC"
            }
         ],
         "name" : "Fanatic Sister",
         "notes" : null,
         "quality" : 5,
         "selectedUpgrades" : [],
         "selectionId" : "kkg0H",
         "size" : 1,
         "specialRules" : [
            {
               "key" : "fearless",
               "name" : "Fearless",
               "rating" : ""
            },
            {
               "id" : 212,
               "key" : "frenzy",
               "name" : "Frenzy"
            }
         ],
         "traits" : [],
         "upgrades" : [
            "H1"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "7oi8zeiqfamiur21",
         "combined" : false,
         "cost" : 15,
         "customName" : "Flagellant Sister",
         "defense" : 5,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : "2",
               "count" : 1,
               "dependencies" : [
                  {
                     "count" : 1,
                     "upgradeInstanceId" : "07WGNk_Xv",
                     "variant" : "replace"
                  }
               ],
               "id" : "4MIc_Zub",
               "label" : "Chainsaw Sword (A2, AP(1))",
               "name" : "Chainsaw Sword",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "CClql4IC"
            }
         ],
         "id" : "R3sC7UX",
         "joinToUnit" : null,
         "loadout" : [
            {
               "content" : [
                  {
                     "count" : 1,
                     "dependencies" : [],
                     "key" : "regeneration",
                     "label" : "Regeneration",
                     "modify" : false,
                     "name" : "Regeneration",
                     "rating" : "",
                     "type" : "ArmyBookRule"
                  }
               ],
               "count" : 1,
               "dependencies" : [],
               "label" : "Flagellants (Regeneration)",
               "name" : "Flagellants",
               "type" : "ArmyBookItem"
            },
            {
               "attacks" : 2,
               "count" : 1,
               "dependencies" : [],
               "id" : "MKdDx8aj",
               "label" : "Dual Arc-Flails (A2, Rending)",
               "name" : "Dual Arc-Flails",
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "label" : "Rending",
                     "modify" : false,
                     "name" : "Rending",
                     "type" : "ArmyBookRule"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "MKdDx8aj"
            }
         ],
         "name" : "Fanatic Sister",
         "notes" : null,
         "quality" : 5,
         "selectedUpgrades" : [
            {
               "instanceId" : "GzKM2TikB",
               "option" : {
                  "cost" : 5,
                  "costs" : [
                     {
                        "cost" : 5,
                        "unitId" : "R3sC7UX"
                     }
                  ],
                  "gains" : [
                     {
                        "content" : [
                           {
                              "count" : 1,
                              "dependencies" : [],
                              "key" : "regeneration",
                              "label" : "Regeneration",
                              "modify" : false,
                              "name" : "Regeneration",
                              "rating" : "",
                              "type" : "ArmyBookRule"
                           }
                        ],
                        "count" : 1,
                        "dependencies" : [],
                        "label" : "Flagellants (Regeneration)",
                        "name" : "Flagellants",
                        "type" : "ArmyBookItem"
                     }
                  ],
                  "id" : "q4Db3",
                  "label" : "Flagellants (Regeneration)",
                  "parentPackageUid" : "H1",
                  "parentSectionUid" : "Zl_L3",
                  "proposedCost" : 10,
                  "proposedCostHint" : [
                     {
                        "isValid" : true,
                        "newCostPrecise" : "2.7",
                        "newCostRounded" : 5,
                        "unitName" : "Fanatic Sister"
                     }
                  ],
                  "proposedVersion" : "2.50",
                  "type" : "ArmyBookUpgradeOption",
                  "uid" : "q4Db3"
               },
               "upgrade" : {
                  "id" : "En44s8X",
                  "label" : "Upgrade with",
                  "options" : null,
                  "parentPackageUid" : "H1",
                  "type" : "ArmyBookUpgradeSection",
                  "uid" : "Zl_L3",
                  "variant" : "upgrade"
               }
            },
            {
               "instanceId" : "07WGNk_Xv",
               "option" : {
                  "cost" : 0,
                  "costs" : [
                     {
                        "cost" : 0,
                        "unitId" : "R3sC7UX"
                     }
                  ],
                  "gains" : [
                     {
                        "attacks" : 2,
                        "count" : 1,
                        "dependencies" : [],
                        "id" : "MKdDx8aj",
                        "label" : "Dual Arc-Flails (A2, Rending)",
                        "name" : "Dual Arc-Flails",
                        "range" : 0,
                        "specialRules" : [
                           {
                              "key" : "rending",
                              "label" : "Rending",
                              "modify" : false,
                              "name" : "Rending",
                              "type" : "ArmyBookRule"
                           }
                        ],
                        "type" : "ArmyBookWeapon",
                        "weaponId" : "MKdDx8aj"
                     }
                  ],
                  "id" : "AwpWO",
                  "label" : "Dual Arc-Flails (A2, Rending)",
                  "parentPackageUid" : "H1",
                  "parentSectionUid" : "kNZAW",
                  "proposedCost" : 25,
                  "proposedCostHint" : [
                     {
                        "isValid" : true,
                        "newCostPrecise" : "-1.3",
                        "newCostRounded" : 0,
                        "unitName" : "Fanatic Sister"
                     }
                  ],
                  "proposedVersion" : "2.50",
                  "type" : "ArmyBookUpgradeOption",
                  "uid" : "AwpWO"
               },
               "upgrade" : {
                  "id" : "qcfrIVu",
                  "label" : "Replace Chainsaw Sword",
                  "options" : null,
                  "parentPackageUid" : "H1",
                  "targets" : [
                     "Chainsaw Sword"
                  ],
                  "type" : "ArmyBookUpgradeSection",
                  "uid" : "kNZAW",
                  "variant" : "replace"
               }
            }
         ],
         "selectionId" : "_cmZM",
         "size" : 1,
         "specialRules" : [
            {
               "key" : "fearless",
               "name" : "Fearless",
               "rating" : ""
            },
            {
               "id" : 212,
               "key" : "frenzy",
               "name" : "Frenzy"
            }
         ],
         "traits" : [],
         "upgrades" : [
            "H1"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "7oi8zeiqfamiur21",
         "combined" : false,
         "cost" : 20,
         "defense" : 4,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "WWqUi2Wx",
               "label" : "CCW (A1)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "4o5CBBZC"
            },
            {
               "attacks" : 1,
               "count" : 1,
               "dependencies" : [
                  {
                     "count" : 1,
                     "upgradeInstanceId" : "lwGGw-_yE",
                     "variant" : "replace"
                  }
               ],
               "id" : "lXOrCYv-",
               "label" : "Heavy Rifle (24\", A1, AP(1))",
               "name" : "Heavy Rifle",
               "originalCount" : 1,
               "range" : 24,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "lXOrCYv-"
            }
         ],
         "id" : "0Bc4px2",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "WWqUi2Wx",
               "label" : "CCW (A1)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "4o5CBBZC"
            },
            {
               "attacks" : 1,
               "count" : 1,
               "dependencies" : [],
               "id" : "gN1LAzCn",
               "label" : "Plasma Rifle (24\", A1, AP(4))",
               "name" : "Plasma Rifle",
               "range" : 24,
               "specialRules" : [
                  {
                     "name" : "AP",
                     "rating" : 4
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "gN1LAzCn"
            }
         ],
         "name" : "Warrior Sister",
         "notes" : null,
         "quality" : 4,
         "selectedUpgrades" : [
            {
               "instanceId" : "lwGGw-_yE",
               "option" : {
                  "cost" : 10,
                  "costs" : [
                     {
                        "cost" : 10,
                        "unitId" : "0Bc4px2"
                     }
                  ],
                  "gains" : [
                     {
                        "attacks" : 1,
                        "count" : 1,
                        "dependencies" : [],
                        "id" : "gN1LAzCn",
                        "label" : "Plasma Rifle (24\", A1, AP(4))",
                        "name" : "Plasma Rifle",
                        "range" : 24,
                        "specialRules" : [
                           {
                              "name" : "AP",
                              "rating" : 4
                           }
                        ],
                        "type" : "ArmyBookWeapon",
                        "weaponId" : "gN1LAzCn"
                     }
                  ],
                  "id" : "OeJC4E7",
                  "label" : "Plasma Rifle (24\", A1, AP(4))",
                  "parentPackageUid" : "E1",
                  "parentSectionUid" : "rwrkjbz",
                  "proposedCost" : 10,
                  "proposedCostHint" : [
                     {
                        "isValid" : true,
                        "newCostPrecise" : "8.0",
                        "newCostRounded" : 10,
                        "unitName" : "Warrior Sister"
                     }
                  ],
                  "proposedVersion" : "2.50",
                  "type" : "ArmyBookUpgradeOption",
                  "uid" : "OeJC4E7"
               },
               "upgrade" : {
                  "id" : "ZSKXp68",
                  "label" : "Replace Heavy Rifle",
                  "options" : null,
                  "parentPackageUid" : "E1",
                  "targets" : [
                     "Heavy Rifle"
                  ],
                  "type" : "ArmyBookUpgradeSection",
                  "uid" : "rwrkjbz",
                  "variant" : "replace"
               }
            }
         ],
         "selectionId" : "uAmW_",
         "size" : 1,
         "specialRules" : [
            {
               "additional" : false,
               "key" : "devout",
               "name" : "Devout",
               "rating" : ""
            }
         ],
         "traits" : [],
         "upgrades" : [
            "E1",
            "cT2sU",
            "vo3TX"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "7oi8zeiqfamiur21",
         "combined" : false,
         "cost" : 20,
         "defense" : 4,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "WWqUi2Wx",
               "label" : "CCW (A1)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "4o5CBBZC"
            },
            {
               "attacks" : 1,
               "count" : 1,
               "dependencies" : [
                  {
                     "count" : 1,
                     "upgradeInstanceId" : "W45js61Z3",
                     "variant" : "replace"
                  }
               ],
               "id" : "lXOrCYv-",
               "label" : "Heavy Rifle (24\", A1, AP(1))",
               "name" : "Heavy Rifle",
               "originalCount" : 1,
               "range" : 24,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "lXOrCYv-"
            }
         ],
         "id" : "0Bc4px2",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "WWqUi2Wx",
               "label" : "CCW (A1)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "4o5CBBZC"
            },
            {
               "attacks" : 1,
               "count" : 1,
               "dependencies" : [],
               "id" : "gN1LAzCn",
               "label" : "Plasma Rifle (24\", A1, AP(4))",
               "name" : "Plasma Rifle",
               "range" : 24,
               "specialRules" : [
                  {
                     "name" : "AP",
                     "rating" : 4
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "gN1LAzCn"
            }
         ],
         "name" : "Warrior Sister",
         "notes" : null,
         "quality" : 4,
         "selectedUpgrades" : [
            {
               "instanceId" : "W45js61Z3",
               "option" : {
                  "cost" : 10,
                  "costs" : [
                     {
                        "cost" : 10,
                        "unitId" : "0Bc4px2"
                     }
                  ],
                  "gains" : [
                     {
                        "attacks" : 1,
                        "count" : 1,
                        "dependencies" : [],
                        "id" : "gN1LAzCn",
                        "label" : "Plasma Rifle (24\", A1, AP(4))",
                        "name" : "Plasma Rifle",
                        "range" : 24,
                        "specialRules" : [
                           {
                              "name" : "AP",
                              "rating" : 4
                           }
                        ],
                        "type" : "ArmyBookWeapon",
                        "weaponId" : "gN1LAzCn"
                     }
                  ],
                  "id" : "OeJC4E7",
                  "label" : "Plasma Rifle (24\", A1, AP(4))",
                  "parentPackageUid" : "E1",
                  "parentSectionUid" : "rwrkjbz",
                  "proposedCost" : 10,
                  "proposedCostHint" : [
                     {
                        "isValid" : true,
                        "newCostPrecise" : "8.0",
                        "newCostRounded" : 10,
                        "unitName" : "Warrior Sister"
                     }
                  ],
                  "proposedVersion" : "2.50",
                  "type" : "ArmyBookUpgradeOption",
                  "uid" : "OeJC4E7"
               },
               "upgrade" : {
                  "id" : "ZSKXp68",
                  "label" : "Replace Heavy Rifle",
                  "options" : null,
                  "parentPackageUid" : "E1",
                  "targets" : [
                     "Heavy Rifle"
                  ],
                  "type" : "ArmyBookUpgradeSection",
                  "uid" : "rwrkjbz",
                  "variant" : "replace"
               }
            }
         ],
         "selectionId" : "UFcfH",
         "size" : 1,
         "specialRules" : [
            {
               "additional" : false,
               "key" : "devout",
               "name" : "Devout",
               "rating" : ""
            }
         ],
         "traits" : [],
         "upgrades" : [
            "E1",
            "cT2sU",
            "vo3TX"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "7oi8zeiqfamiur21",
         "combined" : false,
         "cost" : 25,
         "defense" : 4,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 1,
               "count" : 1,
               "dependencies" : [
                  {
                     "count" : 1,
                     "upgradeInstanceId" : "k_gytvZQi",
                     "variant" : "replace"
                  }
               ],
               "id" : "WWqUi2Wx",
               "label" : "CCW (A1)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "4o5CBBZC"
            },
            {
               "attacks" : 2,
               "count" : 1,
               "dependencies" : [
                  {
                     "count" : 1,
                     "upgradeInstanceId" : "k_gytvZQi",
                     "variant" : "replace"
                  }
               ],
               "id" : "GKm7CCrS",
               "label" : "Twin Heavy Pistol (12\", A2, AP(1))",
               "name" : "Twin Heavy Pistol",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "GKm7CCrS"
            }
         ],
         "id" : "qgESKb_",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 2,
               "count" : 1,
               "dependencies" : [],
               "label" : "Energy Sword (A2, AP(1), Rending)",
               "name" : "Energy Sword",
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "label" : "AP(1)",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : 1,
                     "type" : "ArmyBookRule"
                  },
                  {
                     "key" : "rending",
                     "label" : "Rending",
                     "modify" : false,
                     "name" : "Rending",
                     "type" : "ArmyBookRule"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "0-erIAQg"
            },
            {
               "attacks" : 1,
               "count" : 1,
               "dependencies" : [],
               "id" : "agIlKQ_g",
               "label" : "Flamer Pistol (6\", A1, Blast(3), Reliable)",
               "name" : "Flamer Pistol",
               "range" : 6,
               "specialRules" : [
                  {
                     "key" : "blast",
                     "label" : "Blast(3)",
                     "modify" : false,
                     "name" : "Blast",
                     "rating" : 3,
                     "type" : "ArmyBookRule"
                  },
                  {
                     "key" : "reliable",
                     "label" : "Reliable",
                     "modify" : false,
                     "name" : "Reliable",
                     "type" : "ArmyBookRule"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "agIlKQ_g"
            }
         ],
         "name" : "Pistoleer Sister",
         "notes" : null,
         "quality" : 4,
         "selectedUpgrades" : [
            {
               "instanceId" : "k_gytvZQi",
               "option" : {
                  "cost" : 5,
                  "costs" : [
                     {
                        "cost" : 5,
                        "unitId" : "qgESKb_"
                     }
                  ],
                  "gains" : [
                     {
                        "attacks" : 2,
                        "count" : 1,
                        "dependencies" : [],
                        "label" : "Energy Sword (A2, AP(1), Rending)",
                        "name" : "Energy Sword",
                        "range" : 0,
                        "specialRules" : [
                           {
                              "key" : "ap",
                              "label" : "AP(1)",
                              "modify" : false,
                              "name" : "AP",
                              "rating" : 1,
                              "type" : "ArmyBookRule"
                           },
                           {
                              "key" : "rending",
                              "label" : "Rending",
                              "modify" : false,
                              "name" : "Rending",
                              "type" : "ArmyBookRule"
                           }
                        ],
                        "type" : "ArmyBookWeapon",
                        "weaponId" : "0-erIAQg"
                     },
                     {
                        "attacks" : 1,
                        "count" : 1,
                        "dependencies" : [
                           {
                              "count" : 1,
                              "upgradeInstanceId" : "z9f7EmgGt",
                              "variant" : "replace"
                           }
                        ],
                        "label" : "Sgt. Heavy Pistol (12\", A1, AP(1))",
                        "name" : "Sgt. Heavy Pistol",
                        "range" : 12,
                        "specialRules" : [
                           {
                              "key" : "ap",
                              "label" : "AP(1)",
                              "modify" : false,
                              "name" : "AP",
                              "rating" : 1,
                              "type" : "ArmyBookRule"
                           }
                        ],
                        "type" : "ArmyBookWeapon",
                        "weaponId" : "XhE1E2aJ"
                     }
                  ],
                  "id" : "3pqIMyr",
                  "label" : "Energy Sword (A2, AP(1), Rending), Sgt. Heavy Pistol (12\", A1, AP(1))",
                  "parentPackageUid" : "J1",
                  "parentSectionUid" : "H1Ka5oX",
                  "proposedCostHint" : [
                     {
                        "isValid" : true,
                        "newCostPrecise" : "2.4",
                        "newCostRounded" : 0,
                        "unitName" : "Pistoleer Sister"
                     }
                  ],
                  "proposedVersion" : "2.50",
                  "type" : "ArmyBookUpgradeOption",
                  "uid" : "3pqIMyr"
               },
               "upgrade" : {
                  "id" : "e7tKfVA",
                  "label" : "Replace Twin Heavy Pistols and CCW",
                  "options" : null,
                  "parentPackageUid" : "J1",
                  "targets" : [
                     "Twin Heavy Pistols",
                     "CCW"
                  ],
                  "type" : "ArmyBookUpgradeSection",
                  "uid" : "H1Ka5oX",
                  "variant" : "replace"
               }
            },
            {
               "instanceId" : "z9f7EmgGt",
               "option" : {
                  "cost" : 5,
                  "costs" : [
                     {
                        "cost" : 5,
                        "unitId" : "qgESKb_"
                     }
                  ],
                  "gains" : [
                     {
                        "attacks" : 1,
                        "count" : 1,
                        "dependencies" : [],
                        "id" : "agIlKQ_g",
                        "label" : "Flamer Pistol (6\", A1, Blast(3), Reliable)",
                        "name" : "Flamer Pistol",
                        "range" : 6,
                        "specialRules" : [
                           {
                              "key" : "blast",
                              "label" : "Blast(3)",
                              "modify" : false,
                              "name" : "Blast",
                              "rating" : 3,
                              "type" : "ArmyBookRule"
                           },
                           {
                              "key" : "reliable",
                              "label" : "Reliable",
                              "modify" : false,
                              "name" : "Reliable",
                              "type" : "ArmyBookRule"
                           }
                        ],
                        "type" : "ArmyBookWeapon",
                        "weaponId" : "agIlKQ_g"
                     }
                  ],
                  "id" : "sr134",
                  "label" : "Flamer Pistol (6\", A1, Blast(3), Reliable)",
                  "parentPackageUid" : "J1",
                  "parentSectionUid" : "VIce1",
                  "proposedCost" : 10,
                  "proposedCostHint" : [
                     {
                        "isValid" : true,
                        "newCostPrecise" : "4.9",
                        "newCostRounded" : 5,
                        "unitName" : "Pistoleer Sister"
                     }
                  ],
                  "proposedVersion" : "2.50",
                  "type" : "ArmyBookUpgradeOption",
                  "uid" : "sr134"
               },
               "upgrade" : {
                  "id" : "Asjt4Ev",
                  "label" : "Replace  Sgt. Heavy Pistol",
                  "options" : null,
                  "parentPackageUid" : "J1",
                  "targets" : [
                     "Sgt. Heavy Pistol"
                  ],
                  "type" : "ArmyBookUpgradeSection",
                  "uid" : "VIce1",
                  "variant" : "replace"
               }
            }
         ],
         "selectionId" : "0D6UJ",
         "size" : 1,
         "specialRules" : [
            {
               "key" : "ambush",
               "name" : "Ambush",
               "rating" : ""
            },
            {
               "additional" : false,
               "key" : "devout",
               "name" : "Devout",
               "rating" : ""
            },
            {
               "key" : "flying",
               "name" : "Flying",
               "rating" : ""
            }
         ],
         "traits" : [],
         "upgrades" : [
            "J1"
         ],
         "valid" : true,
         "xp" : 0
      }
   ]
}
"#,
    // nLBrzTpB1TTJ Alien Hives
        r#"
{
   "campaignMode" : false,
   "competitive" : true,
   "gameSystem" : "gff",
   "id" : "nLBrzTpB1TTJ",
   "isCloud" : false,
   "modified" : "2023-11-15T10:42:42.782Z",
   "name" : "Alien Hives 200pts - v0",
   "points" : 175,
   "pointsLimit" : 200,
   "selectedUnitId" : "08grg",
   "specialRules" : [
      {
         "aliasedRuleId" : null,
         "description" : "Once per activation, pick one friendly Caster within 6”, which gets +1 to its next spell casting roll.",
         "hasRating" : false,
         "id" : 263,
         "name" : "Spell Warden"
      },
      {
         "aliasedRuleId" : null,
         "description" : "Once per activation, before attacking, roll one die. On a 2+ one enemy unit within 6” in line of sight takes 1 hit with Blast(3) and AP(1).",
         "hasRating" : false,
         "id" : 62,
         "name" : "Breath Attack"
      },
      {
         "aliasedRuleId" : null,
         "description" : "If this model is ever 1\" away from an enemy unit, it is immediately killed, and the enemy takes X*2 hits. This model automatically passes all morale tests.",
         "hasRating" : true,
         "id" : 96,
         "name" : "Explode"
      },
      {
         "aliasedRuleId" : null,
         "description" : "This model counts as having Ambush, and may be deployed up to 1” away from enemy units. Once deployed, roll X dice. For each 2+ one enemy unit within 3” takes 2 hits with AP(1).",
         "hasRating" : true,
         "id" : 110,
         "name" : "Surprise Attack"
      },
      {
         "aliasedRuleId" : null,
         "description" : "Once per activation, pick 2 friendly units within 6”. Those units, and all friendly units within 6\" get Stealth next time they are shot at.",
         "hasRating" : false,
         "id" : 121,
         "name" : "Shrouding Mist"
      },
      {
         "aliasedRuleId" : null,
         "description" : "Once per game, when this model is activated, you may place a new unit of 6 Spores fully within 6” of it.",
         "hasRating" : false,
         "id" : 899,
         "name" : "Release Spores"
      },
      {
         "aliasedRuleId" : null,
         "description" : "Whenever this model takes a wound in melee, the attacker takes 1 hit.",
         "hasRating" : false,
         "id" : 115,
         "name" : "Corrosive"
      },
      {
         "aliasedRuleId" : null,
         "description" : "For each missed attack you may place a new unit of 3 Spores or 1 Massive Spore 12” away from the target, but the position is decided by your opponent. Note that this new unit can’t be activated on the round in which it is placed.",
         "hasRating" : false,
         "id" : 109,
         "name" : "Spores"
      },
      {
         "aliasedRuleId" : null,
         "description" : "Once per game, when this model is activated, you may place a new unit of 10 Assault Grunts, 10 Shooter Grunts, or 6 Hive Swarms fully within 6” of it.",
         "hasRating" : false,
         "id" : 122,
         "name" : "Spawn Brood"
      },
      {
         "aliasedRuleId" : 424,
         "description" : "Once per activation, before attacking, pick one other friendly unit within 12”, which may move by up to 6\".",
         "hasRating" : false,
         "id" : 424,
         "name" : "Pheromones"
      },
      {
         "aliasedRuleId" : 221,
         "description" : "Pick one model with this rule in your army to count as having Caster(1), which gets +1 spell token per other friendly model with this rule within 12\" each round. If the model is killed, pick another model with this rule in your army to become the next caster.",
         "hasRating" : false,
         "id" : 221,
         "name" : "Psychic Synapse"
      },
      {
         "aliasedRuleId" : null,
         "description" : "Once per game, when this model attacks in melee, you may pick one model in the unit as its target, and make 1 attack at Quality 2+ with AP(1) and Deadly(3), which is resolved as if it's a unit of 1.",
         "hasRating" : false,
         "id" : 97,
         "name" : "Takedown"
      },
      {
         "aliasedRuleId" : 281,
         "description" : "When taking a wound, roll one die, and on a 6+ it is ignored. If the wound was from a spell, then it is ignored on a 4+ instead.",
         "hasRating" : false,
         "id" : 281,
         "name" : "Psy-Barrier"
      },
      {
         "aliasedRuleId" : null,
         "description" : "Whenever this unit fails a morale test, it takes one wound, and the morale test counts as passed instead.",
         "hasRating" : false,
         "id" : 99,
         "name" : "No Retreat"
      },
      {
         "aliasedRuleId" : null,
         "description" : "When taking a wound, roll one die, and on a 6+ it is ignored. If the wound was from a spell, then it is ignored on a 4+ instead.",
         "hasRating" : false,
         "id" : 281,
         "name" : "Resistance"
      }
   ],
   "undoUnitRemove" : [
      {
         "armyId" : "w7qor7b2kuifcyvk",
         "combined" : false,
         "cost" : 65,
         "defense" : 4,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 2,
               "count" : 2,
               "dependencies" : [],
               "id" : "EJxewz_6",
               "label" : "Razor Claws (A2)",
               "name" : "Razor Claws",
               "originalCount" : 2,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "DfidZY7Z"
            }
         ],
         "id" : "tbvpeyi",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 2,
               "count" : 2,
               "dependencies" : [],
               "id" : "EJxewz_6",
               "label" : "Razor Claws (A2)",
               "name" : "Razor Claws",
               "originalCount" : 2,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "DfidZY7Z"
            }
         ],
         "name" : "Shadow Leaper",
         "notes" : null,
         "quality" : 4,
         "selectedUpgrades" : [],
         "selectionId" : "uRu9Z",
         "size" : 1,
         "sortId" : 13,
         "specialRules" : [
            {
               "id" : 60,
               "key" : "ambush",
               "name" : "Ambush"
            },
            {
               "id" : 63,
               "key" : "fast",
               "name" : "Fast"
            },
            {
               "id" : 78,
               "key" : "stealth",
               "name" : "Stealth"
            },
            {
               "id" : 79,
               "key" : "strider",
               "name" : "Strider"
            },
            {
               "id" : 80,
               "key" : "tough",
               "name" : "Tough",
               "rating" : 3
            }
         ],
         "traits" : [],
         "upgrades" : [
            "I1"
         ],
         "valid" : true,
         "xp" : 0
      }
   ],
   "unitPreview" : null,
   "units" : [
      {
         "armyId" : "w7qor7b2kuifcyvk",
         "combined" : false,
         "cost" : 55,
         "defense" : 4,
         "disabledSections" : [
            "cRc_X"
         ],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : "2",
               "count" : 2,
               "id" : "Nr0vHAZZ",
               "label" : "Heavy Razor Claws (A2, AP(1))",
               "name" : "Heavy Razor Claws",
               "originalCount" : 2,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "eun-S-QP"
            }
         ],
         "id" : "jMgimMn",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : "2",
               "count" : 2,
               "id" : "Nr0vHAZZ",
               "label" : "Heavy Razor Claws (A2, AP(1))",
               "name" : "Heavy Razor Claws",
               "originalCount" : 2,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "eun-S-QP"
            },
            {
               "content" : [
                  {
                     "count" : 1,
                     "dependencies" : [],
                     "key" : "ambush",
                     "label" : "Ambush",
                     "modify" : false,
                     "name" : "Ambush",
                     "rating" : "",
                     "type" : "ArmyBookRule"
                  },
                  {
                     "count" : 1,
                     "dependencies" : [],
                     "key" : "flying",
                     "label" : "Flying",
                     "modify" : false,
                     "name" : "Flying",
                     "rating" : "",
                     "type" : "ArmyBookRule"
                  }
               ],
               "count" : 1,
               "dependencies" : [],
               "label" : "Wings (Ambush, Flying)",
               "name" : "Wings",
               "type" : "ArmyBookItem"
            }
         ],
         "name" : "Veteran Warrior",
         "notes" : null,
         "quality" : 4,
         "selectedUpgrades" : [
            {
               "instanceId" : "Is2DV5FgH",
               "option" : {
                  "cost" : 10,
                  "costs" : [
                     {
                        "cost" : 10,
                        "unitId" : "jMgimMn"
                     }
                  ],
                  "gains" : [
                     {
                        "content" : [
                           {
                              "count" : 1,
                              "dependencies" : [],
                              "key" : "ambush",
                              "label" : "Ambush",
                              "modify" : false,
                              "name" : "Ambush",
                              "rating" : "",
                              "type" : "ArmyBookRule"
                           },
                           {
                              "count" : 1,
                              "dependencies" : [],
                              "key" : "flying",
                              "label" : "Flying",
                              "modify" : false,
                              "name" : "Flying",
                              "rating" : "",
                              "type" : "ArmyBookRule"
                           }
                        ],
                        "count" : 1,
                        "dependencies" : [],
                        "label" : "Wings (Ambush, Flying)",
                        "name" : "Wings",
                        "type" : "ArmyBookItem"
                     }
                  ],
                  "id" : "RS4rKkQ",
                  "label" : "Wings (Ambush, Flying)",
                  "parentPackageUid" : "B1",
                  "parentSectionUid" : "ijbm7zF",
                  "proposedCost" : 35,
                  "proposedCostHint" : [
                     {
                        "isValid" : true,
                        "newCostPrecise" : "12.0",
                        "newCostRounded" : 10,
                        "unitName" : "Veteran Warrior"
                     }
                  ],
                  "proposedVersion" : "2.50",
                  "type" : "ArmyBookUpgradeOption",
                  "uid" : "RS4rKkQ"
               },
               "upgrade" : {
                  "id" : "1-QE0LG",
                  "label" : "Upgrade with any",
                  "options" : null,
                  "parentPackageUid" : "B1",
                  "select" : {
                     "type" : "any"
                  },
                  "type" : "ArmyBookUpgradeSection",
                  "uid" : "ijbm7zF",
                  "variant" : "upgrade"
               }
            }
         ],
         "selectionId" : "FTbM-",
         "size" : 1,
         "specialRules" : [
            {
               "key" : "fearless",
               "name" : "Fearless",
               "rating" : ""
            },
            {
               "key" : "hero",
               "name" : "Hero",
               "rating" : ""
            },
            {
               "key" : "tough",
               "name" : "Tough",
               "rating" : 3
            }
         ],
         "traits" : [],
         "upgrades" : [
            "B1",
            "C1",
            "dGM4U"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "w7qor7b2kuifcyvk",
         "combined" : false,
         "cost" : 35,
         "defense" : 5,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "soTcQuYH",
               "label" : "Razor Claws (A1)",
               "name" : "Razor Claws",
               "originalCount" : 1,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "soTcQuYH"
            },
            {
               "attacks" : "1",
               "count" : 1,
               "id" : "UctLyaE6",
               "label" : "Bio-Cannon (24\", A1, Blast(3), Indirect, Rending)",
               "name" : "Bio-Cannon",
               "originalCount" : 1,
               "range" : 24,
               "specialRules" : [
                  {
                     "key" : "blast",
                     "modify" : false,
                     "name" : "Blast",
                     "rating" : "3"
                  },
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  },
                  {
                     "key" : "indirect",
                     "modify" : false,
                     "name" : "Indirect",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "UctLyaE6"
            }
         ],
         "id" : "_c1518l",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "soTcQuYH",
               "label" : "Razor Claws (A1)",
               "name" : "Razor Claws",
               "originalCount" : 1,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "soTcQuYH"
            },
            {
               "attacks" : "1",
               "count" : 1,
               "id" : "UctLyaE6",
               "label" : "Bio-Cannon (24\", A1, Blast(3), Indirect, Rending)",
               "name" : "Bio-Cannon",
               "originalCount" : 1,
               "range" : 24,
               "specialRules" : [
                  {
                     "key" : "blast",
                     "modify" : false,
                     "name" : "Blast",
                     "rating" : "3"
                  },
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  },
                  {
                     "key" : "indirect",
                     "modify" : false,
                     "name" : "Indirect",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "UctLyaE6"
            }
         ],
         "name" : "Support Grunt",
         "notes" : null,
         "quality" : 5,
         "selectedUpgrades" : [],
         "selectionId" : "08grg",
         "size" : 1,
         "specialRules" : [
            {
               "id" : 74,
               "key" : "relentless",
               "name" : "Relentless"
            },
            {
               "id" : 79,
               "key" : "strider",
               "name" : "Strider"
            }
         ],
         "traits" : [],
         "upgrades" : [
            "D1"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "w7qor7b2kuifcyvk",
         "combined" : false,
         "cost" : 40,
         "defense" : 5,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 2,
               "count" : 3,
               "id" : "7GScCDgm",
               "label" : "Bio-Borer (12\", A2)",
               "name" : "Bio-Borer",
               "originalCount" : 3,
               "range" : 12,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "7GScCDgm"
            },
            {
               "attacks" : 1,
               "count" : 3,
               "id" : "soTcQuYH",
               "label" : "Razor Claws (A1)",
               "name" : "Razor Claws",
               "originalCount" : 3,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "soTcQuYH"
            }
         ],
         "id" : "CMADHYi",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 2,
               "count" : 3,
               "id" : "7GScCDgm",
               "label" : "Bio-Borer (12\", A2)",
               "name" : "Bio-Borer",
               "originalCount" : 3,
               "range" : 12,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "7GScCDgm"
            },
            {
               "attacks" : 1,
               "count" : 3,
               "id" : "soTcQuYH",
               "label" : "Razor Claws (A1)",
               "name" : "Razor Claws",
               "originalCount" : 3,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "soTcQuYH"
            }
         ],
         "name" : "Shooter Grunts",
         "notes" : null,
         "quality" : 5,
         "selectedUpgrades" : [],
         "selectionId" : "c9tXb",
         "size" : 3,
         "specialRules" : [
            {
               "key" : "strider",
               "name" : "Strider",
               "rating" : ""
            }
         ],
         "traits" : [],
         "upgrades" : [
            "D1",
            "E1"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "w7qor7b2kuifcyvk",
         "combined" : false,
         "cost" : 35,
         "defense" : 5,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "soTcQuYH",
               "label" : "Razor Claws (A1)",
               "name" : "Razor Claws",
               "originalCount" : 1,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "soTcQuYH"
            },
            {
               "attacks" : "1",
               "count" : 1,
               "id" : "UctLyaE6",
               "label" : "Bio-Cannon (24\", A1, Blast(3), Indirect, Rending)",
               "name" : "Bio-Cannon",
               "originalCount" : 1,
               "range" : 24,
               "specialRules" : [
                  {
                     "key" : "blast",
                     "modify" : false,
                     "name" : "Blast",
                     "rating" : "3"
                  },
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  },
                  {
                     "key" : "indirect",
                     "modify" : false,
                     "name" : "Indirect",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "UctLyaE6"
            }
         ],
         "id" : "_c1518l",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "soTcQuYH",
               "label" : "Razor Claws (A1)",
               "name" : "Razor Claws",
               "originalCount" : 1,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "soTcQuYH"
            },
            {
               "attacks" : "1",
               "count" : 1,
               "id" : "UctLyaE6",
               "label" : "Bio-Cannon (24\", A1, Blast(3), Indirect, Rending)",
               "name" : "Bio-Cannon",
               "originalCount" : 1,
               "range" : 24,
               "specialRules" : [
                  {
                     "key" : "blast",
                     "modify" : false,
                     "name" : "Blast",
                     "rating" : "3"
                  },
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  },
                  {
                     "key" : "indirect",
                     "modify" : false,
                     "name" : "Indirect",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "UctLyaE6"
            }
         ],
         "name" : "Support Grunt",
         "notes" : null,
         "quality" : 5,
         "selectedUpgrades" : [],
         "selectionId" : "CcaEn",
         "size" : 1,
         "specialRules" : [
            {
               "id" : 74,
               "key" : "relentless",
               "name" : "Relentless"
            },
            {
               "id" : 79,
               "key" : "strider",
               "name" : "Strider"
            }
         ],
         "traits" : [],
         "upgrades" : [
            "D1"
         ],
         "valid" : true,
         "xp" : 0
      }
   ]
}
"#,
    // https://army-forge.onepagerules.com/share?id=2HhzjGpcm5m7&name=Haters_tribes
    r#"
{
   "campaignMode" : false,
   "competitive" : true,
   "gameSystem" : "aofs",
   "id" : "2HhzjGpcm5m7",
   "isCloud" : false,
   "modified" : "2023-12-08T23:06:42.085Z",
   "name" : "Haters tribes",
   "points" : 330,
   "pointsLimit" : 300,
   "selectedUnitId" : "14fxz",
   "specialRules" : [
      {
         "aliasedRuleId" : 857,
         "description" : "Attacks targeting units where all models have this rule count as having AP(-1), to a min. of AP(0).",
         "hasRating" : false,
         "id" : 857,
         "name" : "Heavy Shield"
      }
   ],
   "undoUnitRemove" : [
      {
         "armyId" : "lIHGhJbnAUwW3684",
         "combined" : false,
         "cost" : 20,
         "defense" : 5,
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 1,
               "count" : 3,
               "id" : "omvCv",
               "label" : "Axe (A1, Rending)",
               "name" : "Axe",
               "originalCount" : 3,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "Ra9J3UcY"
            }
         ],
         "id" : "L0K3my6",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 1,
               "count" : 3,
               "id" : "omvCv",
               "label" : "Axe (A1, Rending)",
               "name" : "Axe",
               "originalCount" : 3,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "Ra9J3UcY"
            }
         ],
         "name" : "Youngbloods",
         "notes" : null,
         "quality" : 5,
         "selectedUpgrades" : [],
         "selectionId" : "kN8Wk",
         "size" : 3,
         "sortId" : 2,
         "specialRules" : [],
         "traits" : [],
         "upgrades" : [
            "OUHlX"
         ],
         "valid" : true,
         "xp" : 0
      }
   ],
   "unitPreview" : null,
   "units" : [
      {
         "armyId" : "lIHGhJbnAUwW3684",
         "combined" : false,
         "cost" : 40,
         "defense" : 4,
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 1,
               "count" : 3,
               "id" : "oia61",
               "label" : "Axe (A1, Rending)",
               "name" : "Axe",
               "originalCount" : 3,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "Ra9J3UcY"
            }
         ],
         "id" : "i5_1RbO",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 1,
               "count" : 3,
               "id" : "oia61",
               "label" : "Axe (A1, Rending)",
               "name" : "Axe",
               "originalCount" : 3,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "Ra9J3UcY"
            }
         ],
         "name" : "Elite Warrior",
         "notes" : null,
         "quality" : 3,
         "selectedUpgrades" : [],
         "selectionId" : "Z_05d",
         "size" : 3,
         "specialRules" : [],
         "traits" : [],
         "upgrades" : [
            "372fb",
            "OUHlX"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "lIHGhJbnAUwW3684",
         "combined" : false,
         "cost" : 35,
         "defense" : 4,
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 1,
               "count" : 3,
               "dependencies" : [
                  {
                     "count" : 1,
                     "upgradeInstanceId" : "gIYlg6Pjf",
                     "variant" : "replace"
                  },
                  {
                     "count" : 1,
                     "upgradeInstanceId" : "tDHPAdfQU",
                     "variant" : "replace"
                  },
                  {
                     "count" : 1,
                     "upgradeInstanceId" : "YLn4iHuf8",
                     "variant" : "replace"
                  }
               ],
               "id" : "h04D6",
               "label" : "Axe (A1, Rending)",
               "name" : "Axe",
               "originalCount" : 3,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "Ra9J3UcY"
            }
         ],
         "id" : "ezmvvGH",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 2,
               "count" : 1,
               "dependencies" : [],
               "id" : "RXajHxiE",
               "label" : "Twin Hammer (A2, AP(1))",
               "name" : "Twin Hammer",
               "range" : 0,
               "specialRules" : [
                  {
                     "label" : "AP(1)",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : 1,
                     "type" : "ArmyBookRule"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "IqPEVEa4"
            },
            {
               "attacks" : 2,
               "count" : 1,
               "dependencies" : [],
               "id" : "RXajHxiE",
               "label" : "Twin Hammer (A2, AP(1))",
               "name" : "Twin Hammer",
               "range" : 0,
               "specialRules" : [
                  {
                     "label" : "AP(1)",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : 1,
                     "type" : "ArmyBookRule"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "IqPEVEa4"
            },
            {
               "attacks" : 2,
               "count" : 1,
               "dependencies" : [],
               "id" : "myCWbW3h",
               "label" : "Twin Axe (A2, Rending)",
               "name" : "Twin Axe",
               "range" : 0,
               "specialRules" : [
                  {
                     "label" : "Rending",
                     "modify" : false,
                     "name" : "Rending",
                     "type" : "ArmyBookRule"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "OVK26rcy"
            }
         ],
         "name" : "Warrior",
         "notes" : null,
         "quality" : 4,
         "selectedUpgrades" : [
            {
               "instanceId" : "gIYlg6Pjf",
               "option" : {
                  "cost" : 10,
                  "costs" : [
                     {
                        "cost" : 5,
                        "unitId" : "ezmvvGH"
                     },
                     {
                        "cost" : 10,
                        "unitId" : "i5_1RbO"
                     }
                  ],
                  "gains" : [
                     {
                        "attacks" : 2,
                        "count" : 1,
                        "dependencies" : [],
                        "id" : "RXajHxiE",
                        "label" : "Twin Hammer (A2, AP(1))",
                        "name" : "Twin Hammer",
                        "range" : 0,
                        "specialRules" : [
                           {
                              "label" : "AP(1)",
                              "modify" : false,
                              "name" : "AP",
                              "rating" : 1,
                              "type" : "ArmyBookRule"
                           }
                        ],
                        "type" : "ArmyBookWeapon",
                        "weaponId" : "IqPEVEa4"
                     }
                  ],
                  "id" : "vF4VS",
                  "label" : "Twin Hammer (A2, AP(1))",
                  "proposedCostHint" : [
                     {
                        "isValid" : true,
                        "newCostPrecise" : "6.2",
                        "newCostRounded" : 5,
                        "unitName" : "Warrior"
                     },
                     {
                        "isValid" : true,
                        "newCostPrecise" : "8.3",
                        "newCostRounded" : 10,
                        "unitName" : "Elite Warrior"
                     }
                  ],
                  "proposedVersion" : "2.50",
                  "type" : "ArmyBookUpgradeOption",
                  "uid" : "vF4VS"
               },
               "upgrade" : {
                  "affects" : {
                     "type" : "any"
                  },
                  "id" : "hJWKV4Y",
                  "label" : "Replace any Axe",
                  "options" : null,
                  "parentPackageUid" : "OUHlX",
                  "targets" : [
                     "Axe"
                  ],
                  "type" : "ArmyBookUpgradeSection",
                  "uid" : "79ETZ",
                  "variant" : "replace"
               }
            },
            {
               "instanceId" : "tDHPAdfQU",
               "option" : {
                  "cost" : 10,
                  "costs" : [
                     {
                        "cost" : 5,
                        "unitId" : "ezmvvGH"
                     },
                     {
                        "cost" : 10,
                        "unitId" : "i5_1RbO"
                     }
                  ],
                  "gains" : [
                     {
                        "attacks" : 2,
                        "count" : 1,
                        "dependencies" : [],
                        "id" : "RXajHxiE",
                        "label" : "Twin Hammer (A2, AP(1))",
                        "name" : "Twin Hammer",
                        "range" : 0,
                        "specialRules" : [
                           {
                              "label" : "AP(1)",
                              "modify" : false,
                              "name" : "AP",
                              "rating" : 1,
                              "type" : "ArmyBookRule"
                           }
                        ],
                        "type" : "ArmyBookWeapon",
                        "weaponId" : "IqPEVEa4"
                     }
                  ],
                  "id" : "vF4VS",
                  "label" : "Twin Hammer (A2, AP(1))",
                  "proposedCostHint" : [
                     {
                        "isValid" : true,
                        "newCostPrecise" : "6.2",
                        "newCostRounded" : 5,
                        "unitName" : "Warrior"
                     },
                     {
                        "isValid" : true,
                        "newCostPrecise" : "8.3",
                        "newCostRounded" : 10,
                        "unitName" : "Elite Warrior"
                     }
                  ],
                  "proposedVersion" : "2.50",
                  "type" : "ArmyBookUpgradeOption",
                  "uid" : "vF4VS"
               },
               "upgrade" : {
                  "affects" : {
                     "type" : "any"
                  },
                  "id" : "hJWKV4Y",
                  "label" : "Replace any Axe",
                  "options" : null,
                  "parentPackageUid" : "OUHlX",
                  "targets" : [
                     "Axe"
                  ],
                  "type" : "ArmyBookUpgradeSection",
                  "uid" : "79ETZ",
                  "variant" : "replace"
               }
            },
            {
               "instanceId" : "YLn4iHuf8",
               "option" : {
                  "cost" : 5,
                  "costs" : [
                     {
                        "cost" : 5,
                        "unitId" : "ezmvvGH"
                     },
                     {
                        "cost" : 5,
                        "unitId" : "i5_1RbO"
                     }
                  ],
                  "gains" : [
                     {
                        "attacks" : 2,
                        "count" : 1,
                        "dependencies" : [],
                        "id" : "myCWbW3h",
                        "label" : "Twin Axe (A2, Rending)",
                        "name" : "Twin Axe",
                        "range" : 0,
                        "specialRules" : [
                           {
                              "label" : "Rending",
                              "modify" : false,
                              "name" : "Rending",
                              "type" : "ArmyBookRule"
                           }
                        ],
                        "type" : "ArmyBookWeapon",
                        "weaponId" : "OVK26rcy"
                     }
                  ],
                  "id" : "oFHX6",
                  "label" : "Twin Axe (A2, Rending)",
                  "proposedCostHint" : [
                     {
                        "isValid" : true,
                        "newCostPrecise" : "5.0",
                        "newCostRounded" : 5,
                        "unitName" : "Warrior"
                     },
                     {
                        "isValid" : true,
                        "newCostPrecise" : "6.7",
                        "newCostRounded" : 5,
                        "unitName" : "Elite Warrior"
                     }
                  ],
                  "proposedVersion" : "2.50",
                  "type" : "ArmyBookUpgradeOption",
                  "uid" : "oFHX6"
               },
               "upgrade" : {
                  "affects" : {
                     "type" : "any"
                  },
                  "id" : "hJWKV4Y",
                  "label" : "Replace any Axe",
                  "options" : null,
                  "parentPackageUid" : "OUHlX",
                  "targets" : [
                     "Axe"
                  ],
                  "type" : "ArmyBookUpgradeSection",
                  "uid" : "79ETZ",
                  "variant" : "replace"
               }
            }
         ],
         "selectionId" : "EIvTc",
         "size" : 3,
         "specialRules" : [],
         "traits" : [],
         "upgrades" : [
            "OUHlX"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "lIHGhJbnAUwW3684",
         "combined" : false,
         "cost" : 30,
         "defense" : 4,
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "8ZPPT",
               "label" : "Axe (A1, Rending)",
               "name" : "Axe",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "Ra9J3UcY"
            }
         ],
         "id" : "I5_HNrP",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 1,
               "count" : 1,
               "id" : "8ZPPT",
               "label" : "Axe (A1, Rending)",
               "name" : "Axe",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "Ra9J3UcY"
            }
         ],
         "name" : "Champion",
         "notes" : null,
         "quality" : 4,
         "selectedUpgrades" : [],
         "selectionId" : "mWQ0h",
         "size" : 1,
         "specialRules" : [
            {
               "id" : 68,
               "key" : "hero",
               "name" : "Hero"
            },
            {
               "id" : 80,
               "key" : "tough",
               "name" : "Tough",
               "rating" : 3
            }
         ],
         "traits" : [],
         "upgrades" : [],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "lIHGhJbnAUwW3684",
         "combined" : false,
         "cost" : 90,
         "defense" : 3,
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : "2",
               "count" : 1,
               "id" : "P0MHA",
               "label" : "Great Hammer (A2, AP(3))",
               "name" : "Great Hammer",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "3"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "0phUUdFE"
            }
         ],
         "id" : "1FQu45u",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : "2",
               "count" : 1,
               "id" : "P0MHA",
               "label" : "Great Hammer (A2, AP(3))",
               "name" : "Great Hammer",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "3"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "0phUUdFE"
            }
         ],
         "name" : "Prince",
         "notes" : null,
         "quality" : 3,
         "selectedUpgrades" : [],
         "selectionId" : "i45My",
         "size" : 1,
         "specialRules" : [
            {
               "id" : 68,
               "key" : "hero",
               "name" : "Hero"
            },
            {
               "id" : 80,
               "key" : "tough",
               "name" : "Tough",
               "rating" : 6
            }
         ],
         "traits" : [],
         "upgrades" : [
            "tvdI1"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "lIHGhJbnAUwW3684",
         "combined" : false,
         "cost" : 40,
         "defense" : 4,
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 1,
               "count" : 3,
               "id" : "oia61",
               "label" : "Axe (A1, Rending)",
               "name" : "Axe",
               "originalCount" : 3,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "Ra9J3UcY"
            }
         ],
         "id" : "i5_1RbO",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 1,
               "count" : 3,
               "id" : "oia61",
               "label" : "Axe (A1, Rending)",
               "name" : "Axe",
               "originalCount" : 3,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "Ra9J3UcY"
            }
         ],
         "name" : "Elite Warrior",
         "notes" : null,
         "quality" : 3,
         "selectedUpgrades" : [],
         "selectionId" : "yzk_u",
         "size" : 3,
         "specialRules" : [],
         "traits" : [],
         "upgrades" : [
            "372fb",
            "OUHlX"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "lIHGhJbnAUwW3684",
         "combined" : false,
         "cost" : 35,
         "defense" : 4,
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 1,
               "count" : 3,
               "dependencies" : [
                  {
                     "count" : 1,
                     "upgradeInstanceId" : "hG1FMKP5Y",
                     "variant" : "replace"
                  },
                  {
                     "count" : 1,
                     "upgradeInstanceId" : "P6GjljujB",
                     "variant" : "replace"
                  },
                  {
                     "count" : 1,
                     "upgradeInstanceId" : "AJqEo2TQC",
                     "variant" : "replace"
                  }
               ],
               "id" : "h04D6",
               "label" : "Axe (A1, Rending)",
               "name" : "Axe",
               "originalCount" : 3,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "Ra9J3UcY"
            }
         ],
         "id" : "ezmvvGH",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 2,
               "count" : 1,
               "dependencies" : [],
               "label" : "Heavy Axe (A2, AP(1), Rending)",
               "name" : "Heavy Axe",
               "range" : 0,
               "specialRules" : [
                  {
                     "label" : "AP(1)",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : 1,
                     "type" : "ArmyBookRule"
                  },
                  {
                     "label" : "Rending",
                     "modify" : false,
                     "name" : "Rending",
                     "type" : "ArmyBookRule"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "fm65u75F"
            },
            {
               "attacks" : 2,
               "count" : 1,
               "dependencies" : [],
               "label" : "Heavy Axe (A2, AP(1), Rending)",
               "name" : "Heavy Axe",
               "range" : 0,
               "specialRules" : [
                  {
                     "label" : "AP(1)",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : 1,
                     "type" : "ArmyBookRule"
                  },
                  {
                     "label" : "Rending",
                     "modify" : false,
                     "name" : "Rending",
                     "type" : "ArmyBookRule"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "fm65u75F"
            },
            {
               "attacks" : 2,
               "count" : 1,
               "dependencies" : [],
               "id" : "myCWbW3h",
               "label" : "Twin Axe (A2, Rending)",
               "name" : "Twin Axe",
               "range" : 0,
               "specialRules" : [
                  {
                     "label" : "Rending",
                     "modify" : false,
                     "name" : "Rending",
                     "type" : "ArmyBookRule"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "OVK26rcy"
            }
         ],
         "name" : "Warrior",
         "notes" : null,
         "quality" : 4,
         "selectedUpgrades" : [
            {
               "instanceId" : "hG1FMKP5Y",
               "option" : {
                  "cost" : 10,
                  "costs" : [
                     {
                        "cost" : 10,
                        "unitId" : "ezmvvGH"
                     },
                     {
                        "cost" : 10,
                        "unitId" : "i5_1RbO"
                     }
                  ],
                  "gains" : [
                     {
                        "attacks" : 2,
                        "count" : 1,
                        "dependencies" : [],
                        "label" : "Heavy Axe (A2, AP(1), Rending)",
                        "name" : "Heavy Axe",
                        "range" : 0,
                        "specialRules" : [
                           {
                              "label" : "AP(1)",
                              "modify" : false,
                              "name" : "AP",
                              "rating" : 1,
                              "type" : "ArmyBookRule"
                           },
                           {
                              "label" : "Rending",
                              "modify" : false,
                              "name" : "Rending",
                              "type" : "ArmyBookRule"
                           }
                        ],
                        "type" : "ArmyBookWeapon",
                        "weaponId" : "fm65u75F"
                     }
                  ],
                  "id" : "VkdIj",
                  "label" : "Heavy Axe (A2, AP(1), Rending)",
                  "proposedCostHint" : [
                     {
                        "isValid" : true,
                        "newCostPrecise" : "7.6",
                        "newCostRounded" : 10,
                        "unitName" : "Warrior"
                     },
                     {
                        "isValid" : true,
                        "newCostPrecise" : "10.1",
                        "newCostRounded" : 10,
                        "unitName" : "Elite Warrior"
                     }
                  ],
                  "proposedVersion" : "2.50",
                  "type" : "ArmyBookUpgradeOption",
                  "uid" : "VkdIj"
               },
               "upgrade" : {
                  "affects" : {
                     "type" : "any"
                  },
                  "id" : "hJWKV4Y",
                  "label" : "Replace any Axe",
                  "options" : null,
                  "parentPackageUid" : "OUHlX",
                  "targets" : [
                     "Axe"
                  ],
                  "type" : "ArmyBookUpgradeSection",
                  "uid" : "79ETZ",
                  "variant" : "replace"
               }
            },
            {
               "instanceId" : "P6GjljujB",
               "option" : {
                  "cost" : 10,
                  "costs" : [
                     {
                        "cost" : 10,
                        "unitId" : "ezmvvGH"
                     },
                     {
                        "cost" : 10,
                        "unitId" : "i5_1RbO"
                     }
                  ],
                  "gains" : [
                     {
                        "attacks" : 2,
                        "count" : 1,
                        "dependencies" : [],
                        "label" : "Heavy Axe (A2, AP(1), Rending)",
                        "name" : "Heavy Axe",
                        "range" : 0,
                        "specialRules" : [
                           {
                              "label" : "AP(1)",
                              "modify" : false,
                              "name" : "AP",
                              "rating" : 1,
                              "type" : "ArmyBookRule"
                           },
                           {
                              "label" : "Rending",
                              "modify" : false,
                              "name" : "Rending",
                              "type" : "ArmyBookRule"
                           }
                        ],
                        "type" : "ArmyBookWeapon",
                        "weaponId" : "fm65u75F"
                     }
                  ],
                  "id" : "VkdIj",
                  "label" : "Heavy Axe (A2, AP(1), Rending)",
                  "proposedCostHint" : [
                     {
                        "isValid" : true,
                        "newCostPrecise" : "7.6",
                        "newCostRounded" : 10,
                        "unitName" : "Warrior"
                     },
                     {
                        "isValid" : true,
                        "newCostPrecise" : "10.1",
                        "newCostRounded" : 10,
                        "unitName" : "Elite Warrior"
                     }
                  ],
                  "proposedVersion" : "2.50",
                  "type" : "ArmyBookUpgradeOption",
                  "uid" : "VkdIj"
               },
               "upgrade" : {
                  "affects" : {
                     "type" : "any"
                  },
                  "id" : "hJWKV4Y",
                  "label" : "Replace any Axe",
                  "options" : null,
                  "parentPackageUid" : "OUHlX",
                  "targets" : [
                     "Axe"
                  ],
                  "type" : "ArmyBookUpgradeSection",
                  "uid" : "79ETZ",
                  "variant" : "replace"
               }
            },
            {
               "instanceId" : "AJqEo2TQC",
               "option" : {
                  "cost" : 5,
                  "costs" : [
                     {
                        "cost" : 5,
                        "unitId" : "ezmvvGH"
                     },
                     {
                        "cost" : 5,
                        "unitId" : "i5_1RbO"
                     }
                  ],
                  "gains" : [
                     {
                        "attacks" : 2,
                        "count" : 1,
                        "dependencies" : [],
                        "id" : "myCWbW3h",
                        "label" : "Twin Axe (A2, Rending)",
                        "name" : "Twin Axe",
                        "range" : 0,
                        "specialRules" : [
                           {
                              "label" : "Rending",
                              "modify" : false,
                              "name" : "Rending",
                              "type" : "ArmyBookRule"
                           }
                        ],
                        "type" : "ArmyBookWeapon",
                        "weaponId" : "OVK26rcy"
                     }
                  ],
                  "id" : "oFHX6",
                  "label" : "Twin Axe (A2, Rending)",
                  "proposedCostHint" : [
                     {
                        "isValid" : true,
                        "newCostPrecise" : "5.0",
                        "newCostRounded" : 5,
                        "unitName" : "Warrior"
                     },
                     {
                        "isValid" : true,
                        "newCostPrecise" : "6.7",
                        "newCostRounded" : 5,
                        "unitName" : "Elite Warrior"
                     }
                  ],
                  "proposedVersion" : "2.50",
                  "type" : "ArmyBookUpgradeOption",
                  "uid" : "oFHX6"
               },
               "upgrade" : {
                  "affects" : {
                     "type" : "any"
                  },
                  "id" : "hJWKV4Y",
                  "label" : "Replace any Axe",
                  "options" : null,
                  "parentPackageUid" : "OUHlX",
                  "targets" : [
                     "Axe"
                  ],
                  "type" : "ArmyBookUpgradeSection",
                  "uid" : "79ETZ",
                  "variant" : "replace"
               }
            }
         ],
         "selectionId" : "14fxz",
         "size" : 3,
         "specialRules" : [],
         "traits" : [],
         "upgrades" : [
            "OUHlX"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "lIHGhJbnAUwW3684",
         "combined" : false,
         "cost" : 20,
         "defense" : 5,
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "Dx5HN",
               "label" : "Heavy Bash (A2, AP(1))",
               "name" : "Heavy Bash",
               "newWeapon" : false,
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "maonJXwL"
            }
         ],
         "id" : "L0K3my6",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "Dx5HN",
               "label" : "Heavy Bash (A2, AP(1))",
               "name" : "Heavy Bash",
               "newWeapon" : false,
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "maonJXwL"
            }
         ],
         "name" : "Youngblood",
         "notes" : null,
         "quality" : 4,
         "selectedUpgrades" : [],
         "selectionId" : "gtoGE",
         "size" : 1,
         "specialRules" : [
            {
               "id" : 76,
               "key" : "scout",
               "name" : "Scout"
            }
         ],
         "traits" : [],
         "upgrades" : [
            "k_8kk"
         ],
         "valid" : true,
         "xp" : 0
      }
   ]
}
"#,
    // https://army-forge.onepagerules.com/share?id=p2KIbSBOYpSB&name=WH_Imperium%20-%20Imperium
    r#"
{
   "campaignMode" : false,
   "competitive" : true,
   "gameSystem" : "gf",
   "id" : "p2KIbSBOYpSB",
   "isCloud" : false,
   "modified" : "2024-01-02T23:30:32.061Z",
   "name" : "WH Imperium - Imperium",
   "points" : 2620,
   "pointsLimit" : 4000,
   "selectedUnitId" : null,
   "specialRules" : [
      {
         "aliasedRuleId" : 48,
         "description" : "This model and its unit get AP(+1) when shooting.",
         "hasRating" : false,
         "id" : 48,
         "name" : "Precision Shots"
      },
      {
         "aliasedRuleId" : 45,
         "description" : "This model and its unit get +1 to hit when shooting.",
         "hasRating" : false,
         "id" : 45,
         "name" : "Battle Rites"
      },
      {
         "aliasedRuleId" : 857,
         "description" : "Attacks targeting units where all models have this rule count as having AP(-1), to a min. of AP(0).",
         "hasRating" : false,
         "id" : 857,
         "name" : "Heavy Shield"
      },
      {
         "aliasedRuleId" : null,
         "description" : "Once per activation, if within 2” of a model with Tough, roll one die. On a 2+ you may remove D3 wounds from that model.",
         "hasRating" : false,
         "id" : 126,
         "name" : "Repair"
      },
      {
         "aliasedRuleId" : null,
         "description" : "Gets +1 to hit in melee and shooting.",
         "hasRating" : false,
         "id" : 128,
         "name" : "Veteran Infantry"
      },
      {
         "aliasedRuleId" : 561,
         "description" : "This model and its unit get Regeneration.",
         "hasRating" : false,
         "id" : 561,
         "name" : "Medical Training"
      },
      {
         "aliasedRuleId" : 55,
         "description" : "This model and its unit get Furious. If they already had Furious, they get extra hits on unmodified rolls of 5-6 instead.",
         "hasRating" : false,
         "id" : 55,
         "name" : "War Chant"
      },
      {
         "aliasedRuleId" : 358,
         "description" : "Whenever this model moves over enemy units, pick one of them and roll one die, on a 6+ it takes 1 hit.",
         "hasRating" : false,
         "id" : 358,
         "name" : "Cluster Grenades"
      },
      {
         "aliasedRuleId" : 48,
         "description" : "This model and its unit get AP(+1) when shooting.",
         "hasRating" : false,
         "id" : 48,
         "name" : "Canticles"
      },
      {
         "aliasedRuleId" : null,
         "description" : "This weapon ignores cover.",
         "hasRating" : false,
         "id" : 505,
         "name" : "Phosphor"
      },
      {
         "aliasedRuleId" : 52,
         "description" : "This model and its unit move +2” on Advance, and +4” on Rush/Charge.",
         "hasRating" : false,
         "id" : 52,
         "name" : "Psalms"
      },
      {
         "aliasedRuleId" : null,
         "description" : "Once per activation, if within 2” of a model with Tough, roll one die. On a 2+ you may remove D3 wounds from that model.",
         "hasRating" : false,
         "id" : 126,
         "name" : "Repair"
      },
      {
         "aliasedRuleId" : null,
         "description" : "This model gets AP(+2) in melee against units where most models have Tough(3) or higher.",
         "hasRating" : false,
         "id" : 164,
         "name" : "Slayer"
      },
      {
         "aliasedRuleId" : null,
         "description" : "Enemies that roll to block hits from this weapon take one additional wound for each unmodified result of 1 that they roll.",
         "hasRating" : false,
         "id" : 98,
         "name" : "Radiation"
      },
      {
         "aliasedRuleId" : 846,
         "description" : "Unmodified rolls of 6 are multiplied by 2 (only the original hit counts as a 6).",
         "hasRating" : false,
         "id" : 846,
         "name" : "Taser"
      },
      {
         "aliasedRuleId" : null,
         "description" : "When shooting at enemies within 12\", hits from unmodified rolls of 5-6 are multiplied by 2 (only the original hit counts as a 6).",
         "hasRating" : false,
         "id" : 991,
         "name" : "Highly Devout"
      },
      {
         "aliasedRuleId" : 48,
         "description" : "This model and its unit get AP(+1) when shooting.",
         "hasRating" : false,
         "id" : 48,
         "name" : "Spiritual Guidance"
      },
      {
         "aliasedRuleId" : 50,
         "description" : "This model and its unit get Stealth.",
         "hasRating" : false,
         "id" : 50,
         "name" : "Blind Faith"
      },
      {
         "aliasedRuleId" : 47,
         "description" : "This model and its unit get AP(+1) in melee.",
         "hasRating" : false,
         "id" : 47,
         "name" : "War Hymns"
      },
      {
         "aliasedRuleId" : 53,
         "description" : "This model and its unit get +1 to morale test rolls.",
         "hasRating" : false,
         "id" : 53,
         "name" : "Canticle Megaphone"
      },
      {
         "aliasedRuleId" : null,
         "description" : "Counts as having Furious and gets AP(+1) when charging.",
         "hasRating" : false,
         "id" : 212,
         "name" : "Frenzy"
      },
      {
         "aliasedRuleId" : null,
         "description" : "This model gets +1 to defense rolls against non-spell attacks.",
         "hasRating" : false,
         "id" : 858,
         "name" : "Shield Wall"
      },
      {
         "aliasedRuleId" : null,
         "description" : "When shooting at enemies within 12\", hits from unmodified rolls of 6 are multiplied by 2 (only the original hit counts as a 6).",
         "hasRating" : null,
         "id" : 895,
         "name" : "Devout"
      },
      {
         "aliasedRuleId" : null,
         "description" : "Once per activation, pick 2 friendly units within 6”, which get +1 to defense rolls next time they take hits.",
         "hasRating" : false,
         "id" : 417,
         "name" : "Protective Aura"
      },
      {
         "aliasedRuleId" : 128,
         "description" : "Gets +1 to hit in melee and shooting.",
         "hasRating" : false,
         "id" : 128,
         "name" : "Celestial Infantry"
      },
      {
         "aliasedRuleId" : 422,
         "description" : "Once per activation, pick 2 friendly units within 6”, which get +1 to hit next time they fight in melee.",
         "hasRating" : false,
         "id" : 422,
         "name" : "Holy Statue"
      },
      {
         "aliasedRuleId" : 224,
         "description" : "Once per activation, pick 2 friendly units within 12”, which get +1 to their next morale test roll.",
         "hasRating" : false,
         "id" : 224,
         "name" : "Angelic Aura"
      },
      {
         "aliasedRuleId" : 561,
         "description" : "This model and its unit get Regeneration.",
         "hasRating" : false,
         "id" : 561,
         "name" : "Medical Training"
      }
   ],
   "undoUnitRemove" : [
      {
         "armyId" : "7oi8zeiqfamiur21",
         "combined" : false,
         "cost" : 85,
         "defense" : 5,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : "2",
               "count" : 5,
               "id" : "4MIc_Zub",
               "label" : "Chainsaw Sword (A2, AP(1))",
               "name" : "Chainsaw Sword",
               "originalCount" : 5,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "CClql4IC"
            }
         ],
         "id" : "R3sC7UX",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : "2",
               "count" : 5,
               "id" : "4MIc_Zub",
               "label" : "Chainsaw Sword (A2, AP(1))",
               "name" : "Chainsaw Sword",
               "originalCount" : 5,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "CClql4IC"
            }
         ],
         "name" : "Fanatic Sisters",
         "notes" : null,
         "quality" : 5,
         "selectedUpgrades" : [],
         "selectionId" : "X4n20",
         "size" : 5,
         "sortId" : 5,
         "specialRules" : [
            {
               "key" : "fearless",
               "name" : "Fearless",
               "rating" : ""
            },
            {
               "id" : 212,
               "key" : "frenzy",
               "name" : "Frenzy"
            }
         ],
         "traits" : [],
         "upgrades" : [
            "H1"
         ],
         "valid" : true,
         "xp" : 0
      }
   ],
   "unitPreview" : null,
   "units" : [
      {
         "armyId" : "oqnnu0gk8q6hyyny",
         "combined" : false,
         "cost" : 60,
         "customName" : "Archivist",
         "defense" : 3,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 2,
               "count" : 1,
               "dependencies" : [
                  {
                     "count" : 1,
                     "upgradeInstanceId" : "x8TY_qLBw",
                     "variant" : "replace"
                  }
               ],
               "id" : "c3nZkAfH",
               "label" : "CCW (A2)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "83nhlNHF"
            },
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "wfEHdWKY",
               "label" : "Master Heavy Pistol (12\", A2, AP(1))",
               "name" : "Master Heavy Pistol",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "0EpTPX9H"
            }
         ],
         "id" : "qGGJpGt",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "wfEHdWKY",
               "label" : "Master Heavy Pistol (12\", A2, AP(1))",
               "name" : "Master Heavy Pistol",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "0EpTPX9H"
            },
            {
               "content" : [
                  {
                     "count" : 1,
                     "dependencies" : [],
                     "key" : "caster",
                     "label" : "Caster(2)",
                     "name" : "Caster",
                     "rating" : 2,
                     "type" : "ArmyBookRule"
                  }
               ],
               "count" : 1,
               "dependencies" : [],
               "label" : "Archivist (Caster(2))",
               "name" : "Archivist",
               "type" : "ArmyBookItem"
            },
            {
               "attacks" : 2,
               "count" : 1,
               "dependencies" : [],
               "label" : "Energy Sword (A2, AP(1), Rending)",
               "name" : "Energy Sword",
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "label" : "AP(1)",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : 1,
                     "type" : "ArmyBookRule"
                  },
                  {
                     "key" : "rending",
                     "label" : "Rending",
                     "modify" : false,
                     "name" : "Rending",
                     "type" : "ArmyBookRule"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "0-erIAQg"
            },
            {
               "content" : [
                  {
                     "count" : 1,
                     "dependencies" : [],
                     "key" : "scout",
                     "label" : "Scout",
                     "modify" : false,
                     "name" : "Scout",
                     "rating" : "",
                     "type" : "ArmyBookRule"
                  }
               ],
               "count" : 1,
               "dependencies" : [],
               "label" : "Infiltration Gear (Scout)",
               "name" : "Infiltration Gear",
               "type" : "ArmyBookItem"
            }
         ],
         "name" : "Elite Raider",
         "notes" : null,
         "quality" : 4,
         "selectedUpgrades" : [
            {
               "instanceId" : "aJXFpvPi5",
               "option" : {
                  "cost" : 40,
                  "costs" : [
                     {
                        "cost" : 40,
                        "unitId" : "9baVFie"
                     },
                     {
                        "cost" : 40,
                        "unitId" : "Z2WiRfh"
                     },
                     {
                        "cost" : 40,
                        "unitId" : "qGGJpGt"
                     }
                  ],
                  "gains" : [
                     {
                        "content" : [
                           {
                              "count" : 1,
                              "dependencies" : [],
                              "key" : "caster",
                              "label" : "Caster(2)",
                              "name" : "Caster",
                              "rating" : 2,
                              "type" : "ArmyBookRule"
                           }
                        ],
                        "count" : 1,
                        "dependencies" : [],
                        "label" : "Archivist (Caster(2))",
                        "name" : "Archivist",
                        "type" : "ArmyBookItem"
                     }
                  ],
                  "id" : "D3Dqx6h",
                  "label" : "Archivist (Caster(2))",
                  "parentPackageUid" : "C1",
                  "parentSectionUid" : "BCvkhSe",
                  "proposedCost" : 20,
                  "proposedCostHint" : [
                     {
                        "isValid" : true,
                        "newCostPrecise" : "40.0",
                        "newCostRounded" : 40,
                        "unitName" : "Grave Prime Master"
                     },
                     {
                        "isValid" : true,
                        "newCostPrecise" : "40.0",
                        "newCostRounded" : 40,
                        "unitName" : "Prime Master"
                     },
                     {
                        "isValid" : true,
                        "newCostPrecise" : "40.0",
                        "newCostRounded" : 40,
                        "unitName" : "Elite Raider"
                     }
                  ],
                  "proposedVersion" : "2.50",
                  "type" : "ArmyBookUpgradeOption",
                  "uid" : "D3Dqx6h"
               },
               "upgrade" : {
                  "id" : "WQ9pNIW",
                  "isHeroUpgrade" : true,
                  "label" : "Upgrade with one",
                  "options" : null,
                  "parentPackageUid" : "C1",
                  "select" : {
                     "type" : "exactly",
                     "value" : 1
                  },
                  "type" : "ArmyBookUpgradeSection",
                  "uid" : "BCvkhSe",
                  "variant" : "upgrade"
               }
            },
            {
               "instanceId" : "x8TY_qLBw",
               "option" : {
                  "cost" : 5,
                  "costs" : [
                     {
                        "cost" : 5,
                        "unitId" : "Z2WiRfh"
                     },
                     {
                        "cost" : 5,
                        "unitId" : "qGGJpGt"
                     }
                  ],
                  "gains" : [
                     {
                        "attacks" : 2,
                        "count" : 1,
                        "dependencies" : [],
                        "label" : "Energy Sword (A2, AP(1), Rending)",
                        "name" : "Energy Sword",
                        "range" : 0,
                        "specialRules" : [
                           {
                              "key" : "ap",
                              "label" : "AP(1)",
                              "modify" : false,
                              "name" : "AP",
                              "rating" : 1,
                              "type" : "ArmyBookRule"
                           },
                           {
                              "key" : "rending",
                              "label" : "Rending",
                              "modify" : false,
                              "name" : "Rending",
                              "type" : "ArmyBookRule"
                           }
                        ],
                        "type" : "ArmyBookWeapon",
                        "weaponId" : "0-erIAQg"
                     }
                  ],
                  "id" : "OYm4knK",
                  "label" : "Energy Sword (A2, AP(1), Rending)",
                  "parentPackageUid" : "A1",
                  "parentSectionUid" : "MovgoCq",
                  "proposedCostHint" : [
                     {
                        "isValid" : true,
                        "newCostPrecise" : "6.1",
                        "newCostRounded" : 5,
                        "unitName" : "Prime Master"
                     },
                     {
                        "isValid" : true,
                        "newCostPrecise" : "5.1",
                        "newCostRounded" : 5,
                        "unitName" : "Elite Raider"
                     }
                  ],
                  "proposedVersion" : "2.50",
                  "type" : "ArmyBookUpgradeOption",
                  "uid" : "OYm4knK"
               },
               "upgrade" : {
                  "id" : "0wAhUeG",
                  "label" : "Replace  CCW",
                  "options" : null,
                  "parentPackageUid" : "A1",
                  "targets" : [
                     "CCW"
                  ],
                  "type" : "ArmyBookUpgradeSection",
                  "uid" : "MovgoCq",
                  "variant" : "replace"
               }
            },
            {
               "instanceId" : "F7MJ6tYIC",
               "option" : {
                  "cost" : 5,
                  "costs" : [
                     {
                        "cost" : 5,
                        "unitId" : "qGGJpGt"
                     }
                  ],
                  "gains" : [
                     {
                        "content" : [
                           {
                              "count" : 1,
                              "dependencies" : [],
                              "key" : "scout",
                              "label" : "Scout",
                              "modify" : false,
                              "name" : "Scout",
                              "rating" : "",
                              "type" : "ArmyBookRule"
                           }
                        ],
                        "count" : 1,
                        "dependencies" : [],
                        "label" : "Infiltration Gear (Scout)",
                        "name" : "Infiltration Gear",
                        "type" : "ArmyBookItem"
                     }
                  ],
                  "id" : "zOpXO",
                  "label" : "Infiltration Gear (Scout)",
                  "parentPackageUid" : "qIWMF",
                  "parentSectionUid" : "BwY_m",
                  "proposedCost" : 5,
                  "proposedCostHint" : [
                     {
                        "isValid" : true,
                        "newCostPrecise" : "6.0",
                        "newCostRounded" : 5,
                        "unitName" : "Elite Raider"
                     }
                  ],
                  "proposedVersion" : "2.50",
                  "type" : "ArmyBookUpgradeOption",
                  "uid" : "zOpXO"
               },
               "upgrade" : {
                  "id" : "tHiIWY_",
                  "label" : "Upgrade with one",
                  "options" : null,
                  "parentPackageUid" : "qIWMF",
                  "select" : {
                     "type" : "exactly",
                     "value" : 1
                  },
                  "type" : "ArmyBookUpgradeSection",
                  "uid" : "BwY_m",
                  "variant" : "upgrade"
               }
            }
         ],
         "selectionId" : "pnU0c",
         "size" : 1,
         "specialRules" : [
            {
               "key" : "fearless",
               "name" : "Fearless",
               "rating" : ""
            },
            {
               "additional" : false,
               "key" : "furious",
               "name" : "Furious",
               "rating" : ""
            },
            {
               "key" : "hero",
               "name" : "Hero",
               "rating" : ""
            },
            {
               "key" : "strider",
               "name" : "Strider",
               "rating" : ""
            },
            {
               "key" : "tough",
               "name" : "Tough",
               "rating" : "3"
            }
         ],
         "traits" : [],
         "upgrades" : [
            "C1",
            "qIWMF",
            "A1"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "oqnnu0gk8q6hyyny",
         "combined" : false,
         "cost" : 155,
         "defense" : 2,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 2,
               "count" : 5,
               "id" : "c3nZkAfH",
               "label" : "CCW (A2)",
               "name" : "CCW",
               "originalCount" : 5,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "83nhlNHF"
            },
            {
               "attacks" : 1,
               "count" : 5,
               "id" : "wfEHdWKY",
               "label" : "Heavy Pistol (12\", A1, AP(1))",
               "name" : "Heavy Pistol",
               "originalCount" : 5,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "wfEHdWKY"
            }
         ],
         "id" : "hvNiOwU",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 2,
               "count" : 5,
               "id" : "c3nZkAfH",
               "label" : "CCW (A2)",
               "name" : "CCW",
               "originalCount" : 5,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "83nhlNHF"
            },
            {
               "attacks" : 1,
               "count" : 5,
               "id" : "wfEHdWKY",
               "label" : "Heavy Pistol (12\", A1, AP(1))",
               "name" : "Heavy Pistol",
               "originalCount" : 5,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "wfEHdWKY"
            }
         ],
         "name" : "Assault Squad",
         "notes" : null,
         "quality" : 3,
         "selectedUpgrades" : [],
         "selectionId" : "JhW41",
         "size" : 5,
         "specialRules" : [
            {
               "key" : "fearless",
               "name" : "Fearless",
               "rating" : ""
            }
         ],
         "traits" : [],
         "upgrades" : [
            "D1",
            "G1"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "oqnnu0gk8q6hyyny",
         "combined" : false,
         "cost" : 345,
         "defense" : 2,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 4,
               "count" : 3,
               "id" : "O5SZTMlX",
               "label" : "Dual Energy Fists (A4, AP(4))",
               "name" : "Dual Energy Fists",
               "originalCount" : 3,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "4"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "O5SZTMlX"
            },
            {
               "attacks" : 4,
               "count" : 3,
               "id" : "ZiuEfQJI",
               "label" : "Twin Fist-Flamers (6\", A4, Reliable)",
               "name" : "Twin Fist-Flamers",
               "originalCount" : 3,
               "range" : 6,
               "specialRules" : [
                  {
                     "key" : "reliable",
                     "modify" : false,
                     "name" : "Reliable",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "ZqYrIO-o"
            }
         ],
         "id" : "Fz19CEf",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 4,
               "count" : 3,
               "id" : "O5SZTMlX",
               "label" : "Dual Energy Fists (A4, AP(4))",
               "name" : "Dual Energy Fists",
               "originalCount" : 3,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "4"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "O5SZTMlX"
            },
            {
               "attacks" : 4,
               "count" : 3,
               "id" : "ZiuEfQJI",
               "label" : "Twin Fist-Flamers (6\", A4, Reliable)",
               "name" : "Twin Fist-Flamers",
               "originalCount" : 3,
               "range" : 6,
               "specialRules" : [
                  {
                     "key" : "reliable",
                     "modify" : false,
                     "name" : "Reliable",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "ZqYrIO-o"
            }
         ],
         "name" : "Aggro Squad",
         "notes" : null,
         "quality" : 3,
         "selectedUpgrades" : [],
         "selectionId" : "-n-eI",
         "size" : 3,
         "specialRules" : [
            {
               "key" : "fearless",
               "name" : "Fearless",
               "rating" : ""
            },
            {
               "key" : "tough",
               "name" : "Tough",
               "rating" : "3"
            }
         ],
         "traits" : [],
         "upgrades" : [
            "M1"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "oqnnu0gk8q6hyyny",
         "combined" : false,
         "cost" : 75,
         "customName" : "Captain",
         "defense" : 2,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 2,
               "count" : 1,
               "dependencies" : [
                  {
                     "count" : 1,
                     "upgradeInstanceId" : "E0gtLz6ZW",
                     "variant" : "replace"
                  }
               ],
               "id" : "8aOOK33l",
               "label" : "CCW (A2)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "83nhlNHF"
            },
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "lXOrCYv-",
               "label" : "Master Heavy Pistol (12\", A2, AP(1))",
               "name" : "Master Heavy Pistol",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "0EpTPX9H"
            }
         ],
         "id" : "Z2WiRfh",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "lXOrCYv-",
               "label" : "Master Heavy Pistol (12\", A2, AP(1))",
               "name" : "Master Heavy Pistol",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "0EpTPX9H"
            },
            {
               "content" : [
                  {
                     "count" : 1,
                     "dependencies" : [],
                     "key" : "battle-rites",
                     "label" : "Battle Rites",
                     "name" : "Battle Rites",
                     "rating" : "",
                     "type" : "ArmyBookRule"
                  }
               ],
               "count" : 1,
               "dependencies" : [],
               "label" : "Captain (Battle Rites)",
               "name" : "Captain",
               "type" : "ArmyBookItem"
            },
            {
               "attacks" : 1,
               "count" : 1,
               "dependencies" : [],
               "id" : "rb694jW1",
               "label" : "Relic Sword (A1, AP(2), Deadly(3))",
               "name" : "Relic Sword",
               "range" : 0,
               "specialRules" : [
                  {
                     "name" : "AP",
                     "rating" : 2
                  },
                  {
                     "name" : "Deadly",
                     "rating" : 3
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "rb694jW1"
            }
         ],
         "name" : "Prime Master",
         "notes" : null,
         "quality" : 3,
         "selectedUpgrades" : [
            {
               "instanceId" : "Z_zy7N7HO",
               "option" : {
                  "cost" : 50,
                  "costs" : [
                     {
                        "cost" : 50,
                        "unitId" : "9baVFie"
                     },
                     {
                        "cost" : 50,
                        "unitId" : "Z2WiRfh"
                     },
                     {
                        "cost" : 50,
                        "unitId" : "qGGJpGt"
                     }
                  ],
                  "gains" : [
                     {
                        "content" : [
                           {
                              "count" : 1,
                              "dependencies" : [],
                              "key" : "battle-rites",
                              "label" : "Battle Rites",
                              "name" : "Battle Rites",
                              "rating" : "",
                              "type" : "ArmyBookRule"
                           }
                        ],
                        "count" : 1,
                        "dependencies" : [],
                        "label" : "Captain (Battle Rites)",
                        "name" : "Captain",
                        "type" : "ArmyBookItem"
                     }
                  ],
                  "id" : "fwO6jFd",
                  "label" : "Captain (Battle Rites)",
                  "parentPackageUid" : "C1",
                  "parentSectionUid" : "BCvkhSe",
                  "proposedCost" : 70,
                  "proposedCostHint" : [
                     {
                        "isValid" : true,
                        "newCostPrecise" : "47.9",
                        "newCostRounded" : 50,
                        "unitName" : "Grave Prime Master"
                     },
                     {
                        "isValid" : true,
                        "newCostPrecise" : "47.9",
                        "newCostRounded" : 50,
                        "unitName" : "Prime Master"
                     },
                     {
                        "isValid" : true,
                        "newCostPrecise" : "47.9",
                        "newCostRounded" : 50,
                        "unitName" : "Elite Raider"
                     }
                  ],
                  "proposedVersion" : "2.50",
                  "type" : "ArmyBookUpgradeOption",
                  "uid" : "fwO6jFd"
               },
               "upgrade" : {
                  "id" : "WQ9pNIW",
                  "isHeroUpgrade" : true,
                  "label" : "Upgrade with one",
                  "options" : null,
                  "parentPackageUid" : "C1",
                  "select" : {
                     "type" : "exactly",
                     "value" : 1
                  },
                  "type" : "ArmyBookUpgradeSection",
                  "uid" : "BCvkhSe",
                  "variant" : "upgrade"
               }
            },
            {
               "instanceId" : "E0gtLz6ZW",
               "option" : {
                  "cost" : 15,
                  "costs" : [
                     {
                        "cost" : 15,
                        "unitId" : "Z2WiRfh"
                     },
                     {
                        "cost" : 10,
                        "unitId" : "qGGJpGt"
                     }
                  ],
                  "gains" : [
                     {
                        "attacks" : 1,
                        "count" : 1,
                        "dependencies" : [],
                        "id" : "rb694jW1",
                        "label" : "Relic Sword (A1, AP(2), Deadly(3))",
                        "name" : "Relic Sword",
                        "range" : 0,
                        "specialRules" : [
                           {
                              "name" : "AP",
                              "rating" : 2
                           },
                           {
                              "name" : "Deadly",
                              "rating" : 3
                           }
                        ],
                        "type" : "ArmyBookWeapon",
                        "weaponId" : "rb694jW1"
                     }
                  ],
                  "id" : "d6wOTSF",
                  "label" : "Relic Sword (A1, AP(2), Deadly(3))",
                  "parentPackageUid" : "A1",
                  "parentSectionUid" : "MovgoCq",
                  "proposedCost" : 45,
                  "proposedCostHint" : [
                     {
                        "isValid" : true,
                        "newCostPrecise" : "13.3",
                        "newCostRounded" : 15,
                        "unitName" : "Prime Master"
                     },
                     {
                        "isValid" : true,
                        "newCostPrecise" : "11.1",
                        "newCostRounded" : 10,
                        "unitName" : "Elite Raider"
                     }
                  ],
                  "proposedVersion" : "2.50",
                  "type" : "ArmyBookUpgradeOption",
                  "uid" : "d6wOTSF"
               },
               "upgrade" : {
                  "id" : "0wAhUeG",
                  "label" : "Replace  CCW",
                  "options" : null,
                  "parentPackageUid" : "A1",
                  "targets" : [
                     "CCW"
                  ],
                  "type" : "ArmyBookUpgradeSection",
                  "uid" : "MovgoCq",
                  "variant" : "replace"
               }
            }
         ],
         "selectionId" : "aeLIs",
         "size" : 1,
         "specialRules" : [
            {
               "key" : "fearless",
               "name" : "Fearless",
               "rating" : ""
            },
            {
               "key" : "hero",
               "name" : "Hero",
               "rating" : ""
            },
            {
               "key" : "tough",
               "name" : "Tough",
               "rating" : "3"
            }
         ],
         "traits" : [],
         "upgrades" : [
            "A1",
            "B1",
            "C1"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "oqnnu0gk8q6hyyny",
         "combined" : false,
         "cost" : 75,
         "customName" : "Lieutenant",
         "defense" : 2,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 2,
               "count" : 1,
               "dependencies" : [
                  {
                     "count" : 1,
                     "upgradeInstanceId" : "tD-3fvdiO",
                     "variant" : "replace"
                  }
               ],
               "id" : "8aOOK33l",
               "label" : "CCW (A2)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "83nhlNHF"
            },
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "lXOrCYv-",
               "label" : "Master Heavy Pistol (12\", A2, AP(1))",
               "name" : "Master Heavy Pistol",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "0EpTPX9H"
            }
         ],
         "id" : "Z2WiRfh",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "lXOrCYv-",
               "label" : "Master Heavy Pistol (12\", A2, AP(1))",
               "name" : "Master Heavy Pistol",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "0EpTPX9H"
            },
            {
               "content" : [
                  {
                     "count" : 1,
                     "dependencies" : [],
                     "key" : "precision-shots",
                     "label" : "Precision Shots",
                     "name" : "Precision Shots",
                     "rating" : "",
                     "type" : "ArmyBookRule"
                  }
               ],
               "count" : 1,
               "dependencies" : [],
               "label" : "Lieutenant (Precision Shots)",
               "name" : "Lieutenant",
               "type" : "ArmyBookItem"
            },
            {
               "attacks" : 2,
               "count" : 1,
               "dependencies" : [],
               "label" : "Energy Sword (A2, AP(1), Rending)",
               "name" : "Energy Sword",
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "label" : "AP(1)",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : 1,
                     "type" : "ArmyBookRule"
                  },
                  {
                     "key" : "rending",
                     "label" : "Rending",
                     "modify" : false,
                     "name" : "Rending",
                     "type" : "ArmyBookRule"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "0-erIAQg"
            },
            {
               "content" : [
                  {
                     "count" : 1,
                     "dependencies" : [],
                     "key" : "heavy shield",
                     "label" : "Heavy Shield",
                     "modify" : false,
                     "name" : "Heavy Shield",
                     "type" : "ArmyBookRule"
                  }
               ],
               "count" : 1,
               "dependencies" : [],
               "label" : "Combat Shield (Heavy Shield)",
               "name" : "Combat Shield",
               "type" : "ArmyBookItem"
            }
         ],
         "name" : "Prime Master",
         "notes" : null,
         "quality" : 3,
         "selectedUpgrades" : [
            {
               "instanceId" : "YGyABQHqi",
               "option" : {
                  "cost" : 60,
                  "costs" : [
                     {
                        "cost" : 60,
                        "unitId" : "9baVFie"
                     },
                     {
                        "cost" : 60,
                        "unitId" : "Z2WiRfh"
                     },
                     {
                        "cost" : 60,
                        "unitId" : "qGGJpGt"
                     }
                  ],
                  "gains" : [
                     {
                        "content" : [
                           {
                              "count" : 1,
                              "dependencies" : [],
                              "key" : "precision-shots",
                              "label" : "Precision Shots",
                              "name" : "Precision Shots",
                              "rating" : "",
                              "type" : "ArmyBookRule"
                           }
                        ],
                        "count" : 1,
                        "dependencies" : [],
                        "label" : "Lieutenant (Precision Shots)",
                        "name" : "Lieutenant",
                        "type" : "ArmyBookItem"
                     }
                  ],
                  "id" : "VyigG9G",
                  "label" : "Lieutenant (Precision Shots)",
                  "parentPackageUid" : "C1",
                  "parentSectionUid" : "BCvkhSe",
                  "proposedCost" : 85,
                  "proposedCostHint" : [
                     {
                        "isValid" : true,
                        "newCostPrecise" : "57.6",
                        "newCostRounded" : 60,
                        "unitName" : "Grave Prime Master"
                     },
                     {
                        "isValid" : true,
                        "newCostPrecise" : "57.6",
                        "newCostRounded" : 60,
                        "unitName" : "Prime Master"
                     },
                     {
                        "isValid" : true,
                        "newCostPrecise" : "57.6",
                        "newCostRounded" : 60,
                        "unitName" : "Elite Raider"
                     }
                  ],
                  "proposedVersion" : "2.50",
                  "type" : "ArmyBookUpgradeOption",
                  "uid" : "VyigG9G"
               },
               "upgrade" : {
                  "id" : "WQ9pNIW",
                  "isHeroUpgrade" : true,
                  "label" : "Upgrade with one",
                  "options" : null,
                  "parentPackageUid" : "C1",
                  "select" : {
                     "type" : "exactly",
                     "value" : 1
                  },
                  "type" : "ArmyBookUpgradeSection",
                  "uid" : "BCvkhSe",
                  "variant" : "upgrade"
               }
            },
            {
               "instanceId" : "tD-3fvdiO",
               "option" : {
                  "cost" : 5,
                  "costs" : [
                     {
                        "cost" : 5,
                        "unitId" : "Z2WiRfh"
                     },
                     {
                        "cost" : 5,
                        "unitId" : "qGGJpGt"
                     }
                  ],
                  "gains" : [
                     {
                        "attacks" : 2,
                        "count" : 1,
                        "dependencies" : [],
                        "label" : "Energy Sword (A2, AP(1), Rending)",
                        "name" : "Energy Sword",
                        "range" : 0,
                        "specialRules" : [
                           {
                              "key" : "ap",
                              "label" : "AP(1)",
                              "modify" : false,
                              "name" : "AP",
                              "rating" : 1,
                              "type" : "ArmyBookRule"
                           },
                           {
                              "key" : "rending",
                              "label" : "Rending",
                              "modify" : false,
                              "name" : "Rending",
                              "type" : "ArmyBookRule"
                           }
                        ],
                        "type" : "ArmyBookWeapon",
                        "weaponId" : "0-erIAQg"
                     }
                  ],
                  "id" : "OYm4knK",
                  "label" : "Energy Sword (A2, AP(1), Rending)",
                  "parentPackageUid" : "A1",
                  "parentSectionUid" : "MovgoCq",
                  "proposedCostHint" : [
                     {
                        "isValid" : true,
                        "newCostPrecise" : "6.1",
                        "newCostRounded" : 5,
                        "unitName" : "Prime Master"
                     },
                     {
                        "isValid" : true,
                        "newCostPrecise" : "5.1",
                        "newCostRounded" : 5,
                        "unitName" : "Elite Raider"
                     }
                  ],
                  "proposedVersion" : "2.50",
                  "type" : "ArmyBookUpgradeOption",
                  "uid" : "OYm4knK"
               },
               "upgrade" : {
                  "id" : "0wAhUeG",
                  "label" : "Replace  CCW",
                  "options" : null,
                  "parentPackageUid" : "A1",
                  "targets" : [
                     "CCW"
                  ],
                  "type" : "ArmyBookUpgradeSection",
                  "uid" : "MovgoCq",
                  "variant" : "replace"
               }
            },
            {
               "instanceId" : "n870Vue2g",
               "option" : {
                  "cost" : 10,
                  "costs" : [
                     {
                        "cost" : 10,
                        "unitId" : "Z2WiRfh"
                     }
                  ],
                  "gains" : [
                     {
                        "content" : [
                           {
                              "count" : 1,
                              "dependencies" : [],
                              "key" : "heavy shield",
                              "label" : "Heavy Shield",
                              "modify" : false,
                              "name" : "Heavy Shield",
                              "type" : "ArmyBookRule"
                           }
                        ],
                        "count" : 1,
                        "dependencies" : [],
                        "label" : "Combat Shield (Heavy Shield)",
                        "name" : "Combat Shield",
                        "type" : "ArmyBookItem"
                     }
                  ],
                  "id" : "cd-2wsN",
                  "label" : "Combat Shield (Heavy Shield)",
                  "parentPackageUid" : "B1",
                  "parentSectionUid" : "80QB0Hq",
                  "proposedCost" : 5,
                  "proposedCostHint" : [
                     {
                        "isValid" : true,
                        "newCostPrecise" : "7.9",
                        "newCostRounded" : 10,
                        "unitName" : "Prime Master"
                     }
                  ],
                  "proposedVersion" : "2.50",
                  "type" : "ArmyBookUpgradeOption",
                  "uid" : "cd-2wsN"
               },
               "upgrade" : {
                  "id" : "8Jy1CVl",
                  "label" : "Upgrade with one",
                  "options" : null,
                  "parentPackageUid" : "B1",
                  "select" : {
                     "type" : "exactly",
                     "value" : 1
                  },
                  "type" : "ArmyBookUpgradeSection",
                  "uid" : "80QB0Hq",
                  "variant" : "upgrade"
               }
            }
         ],
         "selectionId" : "-KAal",
         "size" : 1,
         "specialRules" : [
            {
               "key" : "fearless",
               "name" : "Fearless",
               "rating" : ""
            },
            {
               "key" : "hero",
               "name" : "Hero",
               "rating" : ""
            },
            {
               "key" : "tough",
               "name" : "Tough",
               "rating" : "3"
            }
         ],
         "traits" : [],
         "upgrades" : [
            "A1",
            "B1",
            "C1"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "7el7k3hgy5pb9o9i",
         "combined" : false,
         "cost" : 90,
         "customName" : "Skitarii Rangers",
         "defense" : 4,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 1,
               "count" : 5,
               "id" : "Uek7nqRw",
               "label" : "CCW (A1)",
               "name" : "CCW",
               "originalCount" : 5,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "4o5CBBZC"
            },
            {
               "attacks" : 1,
               "count" : 5,
               "dependencies" : [
                  {
                     "count" : 1,
                     "upgradeInstanceId" : "dFaxzET4i",
                     "variant" : "replace"
                  }
               ],
               "id" : "u42IQ0oq",
               "label" : "R-Fusil (18\", A1, Radiation)",
               "name" : "R-Fusil",
               "originalCount" : 5,
               "range" : 18,
               "specialRules" : [
                  {
                     "key" : "radiation",
                     "modify" : false,
                     "name" : "Radiation",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "u42IQ0oq"
            }
         ],
         "id" : "L6kGWgk",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 1,
               "count" : 5,
               "id" : "Uek7nqRw",
               "label" : "CCW (A1)",
               "name" : "CCW",
               "originalCount" : 5,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "4o5CBBZC"
            },
            {
               "attacks" : 1,
               "count" : 4,
               "dependencies" : [
                  {
                     "count" : 1,
                     "upgradeInstanceId" : "dFaxzET4i",
                     "variant" : "replace"
                  }
               ],
               "id" : "u42IQ0oq",
               "label" : "R-Fusil (18\", A1, Radiation)",
               "name" : "R-Fusil",
               "originalCount" : 5,
               "range" : 18,
               "specialRules" : [
                  {
                     "key" : "radiation",
                     "modify" : false,
                     "name" : "Radiation",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "u42IQ0oq"
            },
            {
               "attacks" : 1,
               "count" : 1,
               "dependencies" : [],
               "id" : "qbUxSwD4",
               "label" : "Uranium Rifle (30\", A1, AP(1), Radiation, Sniper)",
               "name" : "Uranium Rifle",
               "range" : 30,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "label" : "AP(1)",
                     "name" : "AP",
                     "rating" : "1",
                     "type" : "ArmyBookRule"
                  },
                  {
                     "key" : "radiation",
                     "label" : "Radiation",
                     "name" : "Radiation",
                     "rating" : "",
                     "type" : "ArmyBookRule"
                  },
                  {
                     "key" : "sniper",
                     "label" : "Sniper",
                     "name" : "Sniper",
                     "rating" : "",
                     "type" : "ArmyBookRule"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "qbUxSwD4"
            }
         ],
         "name" : "Cult Rangers",
         "notes" : "- sniper\n- \"tromblon à plasma\" as G-rifle, bah\n- 3 standard",
         "quality" : 4,
         "selectedUpgrades" : [
            {
               "instanceId" : "dFaxzET4i",
               "option" : {
                  "cost" : 35,
                  "costs" : [
                     {
                        "cost" : 35,
                        "unitId" : "L6kGWgk"
                     }
                  ],
                  "gains" : [
                     {
                        "attacks" : 1,
                        "count" : 1,
                        "dependencies" : [],
                        "id" : "qbUxSwD4",
                        "label" : "Uranium Rifle (30\", A1, AP(1), Radiation, Sniper)",
                        "name" : "Uranium Rifle",
                        "range" : 30,
                        "specialRules" : [
                           {
                              "key" : "ap",
                              "label" : "AP(1)",
                              "name" : "AP",
                              "rating" : "1",
                              "type" : "ArmyBookRule"
                           },
                           {
                              "key" : "radiation",
                              "label" : "Radiation",
                              "name" : "Radiation",
                              "rating" : "",
                              "type" : "ArmyBookRule"
                           },
                           {
                              "key" : "sniper",
                              "label" : "Sniper",
                              "name" : "Sniper",
                              "rating" : "",
                              "type" : "ArmyBookRule"
                           }
                        ],
                        "type" : "ArmyBookWeapon",
                        "weaponId" : "qbUxSwD4"
                     }
                  ],
                  "id" : "re-VF2_",
                  "label" : "Uranium Rifle (30\", A1, AP(1), Radiation, Sniper)",
                  "parentPackageUid" : "D1",
                  "parentSectionUid" : "cwC0Lvj",
                  "proposedCost" : 50,
                  "proposedCostHint" : [
                     {
                        "isValid" : true,
                        "newCostPrecise" : "36.9",
                        "newCostRounded" : 35,
                        "unitName" : "Cult Rangers"
                     }
                  ],
                  "proposedVersion" : "2.50",
                  "type" : "ArmyBookUpgradeOption",
                  "uid" : "re-VF2_"
               },
               "upgrade" : {
                  "affects" : {
                     "type" : "exactly",
                     "value" : 1
                  },
                  "id" : "zF7fL4O",
                  "label" : "Replace one R-Fusil",
                  "options" : null,
                  "parentPackageUid" : "D1",
                  "targets" : [
                     "R-Fusil"
                  ],
                  "type" : "ArmyBookUpgradeSection",
                  "uid" : "cwC0Lvj",
                  "variant" : "replace"
               }
            }
         ],
         "selectionId" : "wLPH_",
         "size" : 5,
         "specialRules" : [
            {
               "key" : "strider",
               "name" : "Strider",
               "rating" : ""
            }
         ],
         "traits" : [],
         "upgrades" : [
            "D1",
            "FPgzL"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "oqnnu0gk8q6hyyny",
         "combined" : false,
         "cost" : 155,
         "defense" : 2,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 2,
               "count" : 5,
               "id" : "c3nZkAfH",
               "label" : "CCW (A2)",
               "name" : "CCW",
               "originalCount" : 5,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "83nhlNHF"
            },
            {
               "attacks" : 1,
               "count" : 5,
               "id" : "wfEHdWKY",
               "label" : "Heavy Pistol (12\", A1, AP(1))",
               "name" : "Heavy Pistol",
               "originalCount" : 5,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "wfEHdWKY"
            }
         ],
         "id" : "hvNiOwU",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 2,
               "count" : 5,
               "id" : "c3nZkAfH",
               "label" : "CCW (A2)",
               "name" : "CCW",
               "originalCount" : 5,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "83nhlNHF"
            },
            {
               "attacks" : 1,
               "count" : 5,
               "id" : "wfEHdWKY",
               "label" : "Heavy Pistol (12\", A1, AP(1))",
               "name" : "Heavy Pistol",
               "originalCount" : 5,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "wfEHdWKY"
            }
         ],
         "name" : "Assault Squad",
         "notes" : null,
         "quality" : 3,
         "selectedUpgrades" : [],
         "selectionId" : "aRk5p",
         "size" : 5,
         "specialRules" : [
            {
               "key" : "fearless",
               "name" : "Fearless",
               "rating" : ""
            }
         ],
         "traits" : [],
         "upgrades" : [
            "D1",
            "G1"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "7el7k3hgy5pb9o9i",
         "combined" : false,
         "cost" : 90,
         "customName" : "Skitarii Rangers",
         "defense" : 4,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 1,
               "count" : 5,
               "dependencies" : [
                  {
                     "count" : 1,
                     "upgradeInstanceId" : "65DOviDgH",
                     "variant" : "replace"
                  }
               ],
               "id" : "Uek7nqRw",
               "label" : "CCW (A1)",
               "name" : "CCW",
               "originalCount" : 5,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "4o5CBBZC"
            },
            {
               "attacks" : 1,
               "count" : 5,
               "dependencies" : [
                  {
                     "count" : 1,
                     "upgradeInstanceId" : "Xjv2nGL5p",
                     "variant" : "replace"
                  }
               ],
               "id" : "u42IQ0oq",
               "label" : "R-Fusil (18\", A1, Radiation)",
               "name" : "R-Fusil",
               "originalCount" : 5,
               "range" : 18,
               "specialRules" : [
                  {
                     "key" : "radiation",
                     "modify" : false,
                     "name" : "Radiation",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "u42IQ0oq"
            }
         ],
         "id" : "L6kGWgk",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 1,
               "count" : 5,
               "dependencies" : [
                  {
                     "count" : 1,
                     "upgradeInstanceId" : "65DOviDgH",
                     "variant" : "replace"
                  }
               ],
               "id" : "Uek7nqRw",
               "label" : "CCW (A1)",
               "name" : "CCW",
               "originalCount" : 5,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "4o5CBBZC"
            },
            {
               "attacks" : 1,
               "count" : 4,
               "dependencies" : [
                  {
                     "count" : 1,
                     "upgradeInstanceId" : "Xjv2nGL5p",
                     "variant" : "replace"
                  }
               ],
               "id" : "u42IQ0oq",
               "label" : "R-Fusil (18\", A1, Radiation)",
               "name" : "R-Fusil",
               "originalCount" : 5,
               "range" : 18,
               "specialRules" : [
                  {
                     "key" : "radiation",
                     "modify" : false,
                     "name" : "Radiation",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "u42IQ0oq"
            },
            {
               "attacks" : 2,
               "count" : 1,
               "dependencies" : [],
               "id" : "9n-rb3SG",
               "label" : "Arc Gun (18\", A2, Rending)",
               "name" : "Arc Gun",
               "range" : 18,
               "specialRules" : [
                  {
                     "name" : "Rending"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "9n-rb3SG"
            }
         ],
         "name" : "Cult Rangers",
         "notes" : "- Alpha with CCW A2 + P-Pistol\n- \"enhanced datacable\" ignored, bah\n- \"electro rifle\" as Arc Carbine\n",
         "quality" : 4,
         "selectedUpgrades" : [
            {
               "instanceId" : "65DOviDgH",
               "option" : {
                  "cost" : -5,
                  "costs" : [
                     {
                        "cost" : -5,
                        "unitId" : "iGyqlfa"
                     }
                  ],
                  "gains" : [
                     {
                        "attacks" : "2",
                        "count" : 1,
                        "dependencies" : [
                           {
                              "count" : 1,
                              "upgradeInstanceId" : "pwfRkYj4s",
                              "variant" : "replace"
                           }
                        ],
                        "label" : "Leader R-Pistol (9\", A2, Radiation)",
                        "name" : "Leader R-Pistol",
                        "range" : 9,
                        "specialRules" : [
                           {
                              "label" : "Radiation",
                              "modify" : false,
                              "name" : "Radiation",
                              "type" : "ArmyBookRule"
                           }
                        ],
                        "type" : "ArmyBookWeapon",
                        "weaponId" : "3KYDMUcg"
                     },
                     {
                        "attacks" : "2",
                        "count" : 1,
                        "dependencies" : [],
                        "label" : "Tech-Sword (A2)",
                        "name" : "Tech-Sword",
                        "range" : 0,
                        "specialRules" : [],
                        "type" : "ArmyBookWeapon",
                        "weaponId" : "UBr8TV1n"
                     }
                  ],
                  "id" : "1wkff",
                  "label" : "Leader R-Pistol (9\", A2, Radiation), Tech-Sword (A2)",
                  "parentPackageUid" : "B1",
                  "parentSectionUid" : "LHMyy15",
                  "proposedCostHint" : [
                     {
                        "isValid" : true,
                        "newCostPrecise" : "-3.0",
                        "newCostRounded" : -5,
                        "unitName" : "Cult Leader"
                     }
                  ],
                  "proposedVersion" : "2.50",
                  "type" : "ArmyBookUpgradeOption",
                  "uid" : "1wkff"
               },
               "upgrade" : {
                  "id" : "Bl-YSWb",
                  "label" : "Replace  Leader R-Fusil and CCW",
                  "options" : null,
                  "parentPackageUid" : "B1",
                  "targets" : [
                     "Leader R-Fusil",
                     "CCW"
                  ],
                  "type" : "ArmyBookUpgradeSection",
                  "uid" : "LHMyy15",
                  "variant" : "replace"
               }
            },
            {
               "instanceId" : "pwfRkYj4s",
               "option" : {
                  "cost" : 5,
                  "costs" : [
                     {
                        "cost" : 5,
                        "unitId" : "iGyqlfa"
                     }
                  ],
                  "gains" : [
                     {
                        "attacks" : "3",
                        "count" : 1,
                        "dependencies" : [],
                        "label" : "Leader P-Pistol (9\", A3, Phosphor)",
                        "name" : "Leader P-Pistol",
                        "range" : 9,
                        "specialRules" : [
                           {
                              "label" : "Phosphor",
                              "modify" : false,
                              "name" : "Phosphor",
                              "type" : "ArmyBookRule"
                           }
                        ],
                        "type" : "ArmyBookWeapon",
                        "weaponId" : "TOskpb88"
                     }
                  ],
                  "id" : "aH5568p",
                  "label" : "Leader P-Pistol (9\", A3, Phosphor)",
                  "parentPackageUid" : "B1",
                  "parentSectionUid" : "FdiNqBV",
                  "proposedCostHint" : [
                     {
                        "isValid" : true,
                        "newCostPrecise" : "4.3",
                        "newCostRounded" : 5,
                        "unitName" : "Cult Leader"
                     }
                  ],
                  "proposedVersion" : "2.50",
                  "type" : "ArmyBookUpgradeOption",
                  "uid" : "aH5568p"
               },
               "upgrade" : {
                  "id" : "xBN_7M1",
                  "label" : "Replace  Leader R-Pistol",
                  "options" : null,
                  "parentPackageUid" : "B1",
                  "targets" : [
                     "Leader R-Pistol"
                  ],
                  "type" : "ArmyBookUpgradeSection",
                  "uid" : "FdiNqBV",
                  "variant" : "replace"
               }
            },
            {
               "instanceId" : "Xjv2nGL5p",
               "option" : {
                  "cost" : 10,
                  "costs" : [
                     {
                        "cost" : 10,
                        "unitId" : "L6kGWgk"
                     }
                  ],
                  "gains" : [
                     {
                        "attacks" : 2,
                        "count" : 1,
                        "dependencies" : [],
                        "id" : "9n-rb3SG",
                        "label" : "Arc Gun (18\", A2, Rending)",
                        "name" : "Arc Gun",
                        "range" : 18,
                        "specialRules" : [
                           {
                              "name" : "Rending"
                           }
                        ],
                        "type" : "ArmyBookWeapon",
                        "weaponId" : "9n-rb3SG"
                     }
                  ],
                  "id" : "nTanhQn",
                  "label" : "Arc Gun (18\", A2, Rending)",
                  "parentPackageUid" : "D1",
                  "parentSectionUid" : "cwC0Lvj",
                  "proposedCostHint" : [
                     {
                        "isValid" : true,
                        "newCostPrecise" : "8.0",
                        "newCostRounded" : 10,
                        "unitName" : "Cult Rangers"
                     }
                  ],
                  "proposedVersion" : "2.50",
                  "type" : "ArmyBookUpgradeOption",
                  "uid" : "nTanhQn"
               },
               "upgrade" : {
                  "affects" : {
                     "type" : "exactly",
                     "value" : 1
                  },
                  "id" : "zF7fL4O",
                  "label" : "Replace one R-Fusil",
                  "options" : null,
                  "parentPackageUid" : "D1",
                  "targets" : [
                     "R-Fusil"
                  ],
                  "type" : "ArmyBookUpgradeSection",
                  "uid" : "cwC0Lvj",
                  "variant" : "replace"
               }
            }
         ],
         "selectionId" : "-4WpA",
         "size" : 5,
         "specialRules" : [
            {
               "key" : "strider",
               "name" : "Strider",
               "rating" : ""
            }
         ],
         "traits" : [],
         "upgrades" : [
            "D1",
            "FPgzL"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "7el7k3hgy5pb9o9i",
         "combined" : false,
         "cost" : 80,
         "customName" : "Tech Priest Dominus",
         "defense" : 3,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 3,
               "count" : 1,
               "id" : "yemqTC6X",
               "label" : "Axe-Halberd (A3, AP(4))",
               "name" : "Axe-Halberd",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "4"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "yemqTC6X"
            }
         ],
         "id" : "4ajpg-M",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 3,
               "count" : 1,
               "id" : "yemqTC6X",
               "label" : "Axe-Halberd (A3, AP(4))",
               "name" : "Axe-Halberd",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "4"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "yemqTC6X"
            },
            {
               "content" : [
                  {
                     "count" : 1,
                     "dependencies" : [],
                     "key" : "caster",
                     "label" : "Caster(2)",
                     "name" : "Caster",
                     "rating" : 2,
                     "type" : "ArmyBookRule"
                  }
               ],
               "count" : 1,
               "dependencies" : [],
               "label" : "Machine Lore (Caster(2))",
               "name" : "Machine Lore",
               "type" : "ArmyBookItem"
            }
         ],
         "name" : "High Priest",
         "notes" : null,
         "quality" : 3,
         "selectedUpgrades" : [
            {
               "instanceId" : "mISVP0OcX",
               "option" : {
                  "cost" : 40,
                  "costs" : [
                     {
                        "cost" : 40,
                        "unitId" : "4ajpg-M"
                     },
                     {
                        "cost" : 40,
                        "unitId" : "hvRc6eE"
                     },
                     {
                        "cost" : 40,
                        "unitId" : "iGyqlfa"
                     }
                  ],
                  "gains" : [
                     {
                        "content" : [
                           {
                              "count" : 1,
                              "dependencies" : [],
                              "key" : "caster",
                              "label" : "Caster(2)",
                              "name" : "Caster",
                              "rating" : 2,
                              "type" : "ArmyBookRule"
                           }
                        ],
                        "count" : 1,
                        "dependencies" : [],
                        "label" : "Machine Lore (Caster(2))",
                        "name" : "Machine Lore",
                        "type" : "ArmyBookItem"
                     }
                  ],
                  "id" : "VC9Z-lM",
                  "label" : "Machine Lore (Caster(2))",
                  "parentPackageUid" : "C1",
                  "parentSectionUid" : "ilLvqMc",
                  "proposedCost" : 20,
                  "proposedCostHint" : [
                     {
                        "isValid" : true,
                        "newCostPrecise" : "40.0",
                        "newCostRounded" : 40,
                        "unitName" : "High Priest"
                     },
                     {
                        "isValid" : true,
                        "newCostPrecise" : "40.0",
                        "newCostRounded" : 40,
                        "unitName" : "Sect Leader"
                     },
                     {
                        "isValid" : true,
                        "newCostPrecise" : "40.0",
                        "newCostRounded" : 40,
                        "unitName" : "Cult Leader"
                     }
                  ],
                  "proposedVersion" : "2.50",
                  "type" : "ArmyBookUpgradeOption",
                  "uid" : "VC9Z-lM"
               },
               "upgrade" : {
                  "id" : "pEVYJKT",
                  "isHeroUpgrade" : true,
                  "label" : "Upgrade with one",
                  "options" : null,
                  "parentPackageUid" : "C1",
                  "select" : {
                     "type" : "exactly",
                     "value" : 1
                  },
                  "type" : "ArmyBookUpgradeSection",
                  "uid" : "ilLvqMc",
                  "variant" : "upgrade"
               }
            }
         ],
         "selectionId" : "BQ2cc",
         "size" : 1,
         "specialRules" : [
            {
               "key" : "hero",
               "name" : "Hero",
               "rating" : ""
            },
            {
               "key" : "regeneration",
               "name" : "Regeneration",
               "rating" : ""
            },
            {
               "key" : "tough",
               "name" : "Tough",
               "rating" : "3"
            }
         ],
         "traits" : [],
         "upgrades" : [
            "A1",
            "C1"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "7el7k3hgy5pb9o9i",
         "combined" : false,
         "cost" : 80,
         "customName" : "Tech Priest Enginseer",
         "defense" : 3,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 3,
               "count" : 1,
               "id" : "yemqTC6X",
               "label" : "Axe-Halberd (A3, AP(4))",
               "name" : "Axe-Halberd",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "4"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "yemqTC6X"
            }
         ],
         "id" : "4ajpg-M",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 3,
               "count" : 1,
               "id" : "yemqTC6X",
               "label" : "Axe-Halberd (A3, AP(4))",
               "name" : "Axe-Halberd",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "4"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "yemqTC6X"
            },
            {
               "content" : [
                  {
                     "count" : 1,
                     "dependencies" : [],
                     "key" : "repair",
                     "label" : "Repair",
                     "name" : "Repair",
                     "rating" : "",
                     "type" : "ArmyBookRule"
                  }
               ],
               "count" : 1,
               "dependencies" : [],
               "label" : "Engineer (Repair)",
               "name" : "Engineer",
               "type" : "ArmyBookItem"
            }
         ],
         "name" : "High Priest",
         "notes" : null,
         "quality" : 3,
         "selectedUpgrades" : [
            {
               "instanceId" : "SE_aGKOSJ",
               "option" : {
                  "cost" : 15,
                  "costs" : [
                     {
                        "cost" : 15,
                        "unitId" : "4ajpg-M"
                     },
                     {
                        "cost" : 15,
                        "unitId" : "hvRc6eE"
                     },
                     {
                        "cost" : 15,
                        "unitId" : "iGyqlfa"
                     }
                  ],
                  "gains" : [
                     {
                        "content" : [
                           {
                              "count" : 1,
                              "dependencies" : [],
                              "key" : "repair",
                              "label" : "Repair",
                              "name" : "Repair",
                              "rating" : "",
                              "type" : "ArmyBookRule"
                           }
                        ],
                        "count" : 1,
                        "dependencies" : [],
                        "label" : "Engineer (Repair)",
                        "name" : "Engineer",
                        "type" : "ArmyBookItem"
                     }
                  ],
                  "id" : "WMrFSXc",
                  "label" : "Engineer (Repair)",
                  "parentPackageUid" : "C1",
                  "parentSectionUid" : "ilLvqMc",
                  "proposedCost" : 20,
                  "proposedCostHint" : [
                     {
                        "isValid" : true,
                        "newCostPrecise" : "14.4",
                        "newCostRounded" : 15,
                        "unitName" : "High Priest"
                     },
                     {
                        "isValid" : true,
                        "newCostPrecise" : "14.4",
                        "newCostRounded" : 15,
                        "unitName" : "Sect Leader"
                     },
                     {
                        "isValid" : true,
                        "newCostPrecise" : "14.4",
                        "newCostRounded" : 15,
                        "unitName" : "Cult Leader"
                     }
                  ],
                  "proposedVersion" : "2.50",
                  "type" : "ArmyBookUpgradeOption",
                  "uid" : "WMrFSXc"
               },
               "upgrade" : {
                  "id" : "pEVYJKT",
                  "isHeroUpgrade" : true,
                  "label" : "Upgrade with one",
                  "options" : null,
                  "parentPackageUid" : "C1",
                  "select" : {
                     "type" : "exactly",
                     "value" : 1
                  },
                  "type" : "ArmyBookUpgradeSection",
                  "uid" : "ilLvqMc",
                  "variant" : "upgrade"
               }
            }
         ],
         "selectionId" : "v1nNM",
         "size" : 1,
         "specialRules" : [
            {
               "key" : "hero",
               "name" : "Hero",
               "rating" : ""
            },
            {
               "key" : "regeneration",
               "name" : "Regeneration",
               "rating" : ""
            },
            {
               "key" : "tough",
               "name" : "Tough",
               "rating" : "3"
            }
         ],
         "traits" : [],
         "upgrades" : [
            "A1",
            "C1"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "7el7k3hgy5pb9o9i",
         "combined" : false,
         "cost" : 220,
         "defense" : 4,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 3,
               "count" : 3,
               "id" : "42cIE5Xd",
               "label" : "Cyborg Claw (A3)",
               "name" : "Cyborg Claw",
               "originalCount" : 3,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "42cIE5Xd"
            },
            {
               "attacks" : 4,
               "count" : 3,
               "id" : "LYTfLxn3",
               "label" : "Gravity Cannon (24\", A4, Rending)",
               "name" : "Gravity Cannon",
               "originalCount" : 3,
               "range" : 24,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "LYTfLxn3"
            }
         ],
         "id" : "M2Yr_Lr",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 3,
               "count" : 3,
               "id" : "42cIE5Xd",
               "label" : "Cyborg Claw (A3)",
               "name" : "Cyborg Claw",
               "originalCount" : 3,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "42cIE5Xd"
            },
            {
               "attacks" : 4,
               "count" : 3,
               "id" : "LYTfLxn3",
               "label" : "Gravity Cannon (24\", A4, Rending)",
               "name" : "Gravity Cannon",
               "originalCount" : 3,
               "range" : 24,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "LYTfLxn3"
            }
         ],
         "name" : "Destroyer Cyborgs",
         "notes" : null,
         "quality" : 4,
         "selectedUpgrades" : [],
         "selectionId" : "LPMG2",
         "size" : 3,
         "specialRules" : [
            {
               "key" : "relentless",
               "name" : "Relentless",
               "rating" : ""
            },
            {
               "key" : "slow",
               "name" : "Slow",
               "rating" : ""
            },
            {
               "key" : "tough",
               "name" : "Tough",
               "rating" : "3"
            }
         ],
         "traits" : [],
         "upgrades" : [
            "K1"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "7oi8zeiqfamiur21",
         "combined" : false,
         "cost" : 45,
         "defense" : 4,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "WWqUi2Wx",
               "label" : "CCW (A2)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "83nhlNHF"
            },
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "lXOrCYv-",
               "label" : "Master Heavy Pistol (12\", A2, AP(1))",
               "name" : "Master Heavy Pistol",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "r63zpLTC"
            }
         ],
         "id" : "2TXWi3B",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "WWqUi2Wx",
               "label" : "CCW (A2)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "83nhlNHF"
            },
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "lXOrCYv-",
               "label" : "Master Heavy Pistol (12\", A2, AP(1))",
               "name" : "Master Heavy Pistol",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "r63zpLTC"
            },
            {
               "content" : [
                  {
                     "count" : 1,
                     "dependencies" : [],
                     "key" : "spiritual guidance",
                     "label" : "Spiritual Guidance",
                     "modify" : false,
                     "name" : "Spiritual Guidance",
                     "rating" : "",
                     "type" : "ArmyBookRule"
                  }
               ],
               "count" : 1,
               "dependencies" : [],
               "label" : "Canoness (Spiritual Guidance)",
               "name" : "Canoness",
               "type" : "ArmyBookItem"
            }
         ],
         "name" : "High Sister",
         "notes" : null,
         "quality" : 4,
         "selectedUpgrades" : [
            {
               "instanceId" : "4A9N_MtTB",
               "option" : {
                  "cost" : 60,
                  "costs" : [
                     {
                        "cost" : 60,
                        "unitId" : "sF4ltO_"
                     },
                     {
                        "cost" : 60,
                        "unitId" : "2TXWi3B"
                     },
                     {
                        "cost" : 60,
                        "unitId" : "_nVdDbE"
                     }
                  ],
                  "gains" : [
                     {
                        "content" : [
                           {
                              "count" : 1,
                              "dependencies" : [],
                              "key" : "spiritual guidance",
                              "label" : "Spiritual Guidance",
                              "modify" : false,
                              "name" : "Spiritual Guidance",
                              "rating" : "",
                              "type" : "ArmyBookRule"
                           }
                        ],
                        "count" : 1,
                        "dependencies" : [],
                        "label" : "Canoness (Spiritual Guidance)",
                        "name" : "Canoness",
                        "type" : "ArmyBookItem"
                     }
                  ],
                  "id" : "vdVRg",
                  "label" : "Canoness (Spiritual Guidance)",
                  "parentPackageUid" : "B1",
                  "parentSectionUid" : "jDPiWW0",
                  "proposedCost" : 85,
                  "proposedCostHint" : [
                     {
                        "isValid" : true,
                        "newCostPrecise" : "57.6",
                        "newCostRounded" : 60,
                        "unitName" : "High Destroyer Sister"
                     },
                     {
                        "isValid" : true,
                        "newCostPrecise" : "57.6",
                        "newCostRounded" : 60,
                        "unitName" : "High Sister"
                     },
                     {
                        "isValid" : true,
                        "newCostPrecise" : "57.6",
                        "newCostRounded" : 60,
                        "unitName" : "Novice Leader"
                     }
                  ],
                  "proposedVersion" : "2.50",
                  "type" : "ArmyBookUpgradeOption",
                  "uid" : "vdVRg"
               },
               "upgrade" : {
                  "id" : "8knB57J",
                  "isHeroUpgrade" : true,
                  "label" : "Upgrade with one",
                  "options" : null,
                  "parentPackageUid" : "B1",
                  "select" : {
                     "type" : "exactly",
                     "value" : 1
                  },
                  "type" : "ArmyBookUpgradeSection",
                  "uid" : "jDPiWW0",
                  "variant" : "upgrade"
               }
            }
         ],
         "selectionId" : "vsP-7",
         "size" : 1,
         "specialRules" : [
            {
               "additional" : false,
               "key" : "devout",
               "name" : "Devout",
               "rating" : ""
            },
            {
               "key" : "hero",
               "name" : "Hero",
               "rating" : ""
            },
            {
               "key" : "tough",
               "name" : "Tough",
               "rating" : "3"
            }
         ],
         "traits" : [],
         "upgrades" : [
            "A1",
            "B1",
            "C1"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "7oi8zeiqfamiur21",
         "combined" : false,
         "cost" : 110,
         "defense" : 4,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 1,
               "count" : 5,
               "id" : "WWqUi2Wx",
               "label" : "CCW (A1)",
               "name" : "CCW",
               "originalCount" : 5,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "4o5CBBZC"
            },
            {
               "attacks" : 1,
               "count" : 5,
               "id" : "lXOrCYv-",
               "label" : "Heavy Rifle (24\", A1, AP(1))",
               "name" : "Heavy Rifle",
               "originalCount" : 5,
               "range" : 24,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "lXOrCYv-"
            }
         ],
         "id" : "0Bc4px2",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 1,
               "count" : 5,
               "id" : "WWqUi2Wx",
               "label" : "CCW (A1)",
               "name" : "CCW",
               "originalCount" : 5,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "4o5CBBZC"
            },
            {
               "attacks" : 1,
               "count" : 5,
               "id" : "lXOrCYv-",
               "label" : "Heavy Rifle (24\", A1, AP(1))",
               "name" : "Heavy Rifle",
               "originalCount" : 5,
               "range" : 24,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "lXOrCYv-"
            }
         ],
         "name" : "Warrior Sisters",
         "notes" : null,
         "quality" : 4,
         "selectedUpgrades" : [],
         "selectionId" : "6Jiek",
         "size" : 5,
         "specialRules" : [
            {
               "additional" : false,
               "key" : "devout",
               "name" : "Devout",
               "rating" : ""
            }
         ],
         "traits" : [],
         "upgrades" : [
            "E1",
            "cT2sU",
            "vo3TX"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "7oi8zeiqfamiur21",
         "combined" : false,
         "cost" : 85,
         "customName" : "Repentia Sisters",
         "defense" : 5,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : "2",
               "count" : 5,
               "id" : "4MIc_Zub",
               "label" : "Chainsaw Sword (A2, AP(1))",
               "name" : "Chainsaw Sword",
               "originalCount" : 5,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "CClql4IC"
            }
         ],
         "id" : "R3sC7UX",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : "2",
               "count" : 5,
               "id" : "4MIc_Zub",
               "label" : "Chainsaw Sword (A2, AP(1))",
               "name" : "Chainsaw Sword",
               "originalCount" : 5,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "CClql4IC"
            }
         ],
         "name" : "Fanatic Sisters",
         "notes" : null,
         "quality" : 5,
         "selectedUpgrades" : [],
         "selectionId" : "JtW1o",
         "size" : 5,
         "specialRules" : [
            {
               "key" : "fearless",
               "name" : "Fearless",
               "rating" : ""
            },
            {
               "id" : 212,
               "key" : "frenzy",
               "name" : "Frenzy"
            }
         ],
         "traits" : [],
         "upgrades" : [
            "H1"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "7oi8zeiqfamiur21",
         "combined" : false,
         "cost" : 130,
         "defense" : 4,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 1,
               "count" : 5,
               "dependencies" : [
                  {
                     "count" : 1,
                     "upgradeInstanceId" : "woDAkirCY",
                     "variant" : "replace"
                  }
               ],
               "id" : "WWqUi2Wx",
               "label" : "CCW (A1)",
               "name" : "CCW",
               "originalCount" : 5,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "4o5CBBZC"
            },
            {
               "attacks" : 2,
               "count" : 5,
               "dependencies" : [
                  {
                     "count" : 1,
                     "upgradeInstanceId" : "woDAkirCY",
                     "variant" : "replace"
                  }
               ],
               "id" : "GKm7CCrS",
               "label" : "Twin Heavy Pistol (12\", A2, AP(1))",
               "name" : "Twin Heavy Pistol",
               "originalCount" : 5,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "GKm7CCrS"
            }
         ],
         "id" : "qgESKb_",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 1,
               "count" : 4,
               "dependencies" : [
                  {
                     "count" : 1,
                     "upgradeInstanceId" : "woDAkirCY",
                     "variant" : "replace"
                  }
               ],
               "id" : "WWqUi2Wx",
               "label" : "CCW (A1)",
               "name" : "CCW",
               "originalCount" : 5,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "4o5CBBZC"
            },
            {
               "attacks" : 2,
               "count" : 4,
               "dependencies" : [
                  {
                     "count" : 1,
                     "upgradeInstanceId" : "woDAkirCY",
                     "variant" : "replace"
                  }
               ],
               "id" : "GKm7CCrS",
               "label" : "Twin Heavy Pistol (12\", A2, AP(1))",
               "name" : "Twin Heavy Pistol",
               "originalCount" : 5,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "GKm7CCrS"
            },
            {
               "attacks" : 2,
               "count" : 1,
               "dependencies" : [],
               "label" : "Energy Sword (A2, AP(1), Rending)",
               "name" : "Energy Sword",
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "label" : "AP(1)",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : 1,
                     "type" : "ArmyBookRule"
                  },
                  {
                     "key" : "rending",
                     "label" : "Rending",
                     "modify" : false,
                     "name" : "Rending",
                     "type" : "ArmyBookRule"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "0-erIAQg"
            },
            {
               "attacks" : 1,
               "count" : 1,
               "dependencies" : [],
               "label" : "Sgt. Heavy Pistol (12\", A1, AP(1))",
               "name" : "Sgt. Heavy Pistol",
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "label" : "AP(1)",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : 1,
                     "type" : "ArmyBookRule"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "XhE1E2aJ"
            }
         ],
         "name" : "Pistoleer Sisters",
         "notes" : null,
         "quality" : 4,
         "selectedUpgrades" : [
            {
               "instanceId" : "woDAkirCY",
               "option" : {
                  "cost" : 5,
                  "costs" : [
                     {
                        "cost" : 5,
                        "unitId" : "qgESKb_"
                     }
                  ],
                  "gains" : [
                     {
                        "attacks" : 2,
                        "count" : 1,
                        "dependencies" : [],
                        "label" : "Energy Sword (A2, AP(1), Rending)",
                        "name" : "Energy Sword",
                        "range" : 0,
                        "specialRules" : [
                           {
                              "key" : "ap",
                              "label" : "AP(1)",
                              "modify" : false,
                              "name" : "AP",
                              "rating" : 1,
                              "type" : "ArmyBookRule"
                           },
                           {
                              "key" : "rending",
                              "label" : "Rending",
                              "modify" : false,
                              "name" : "Rending",
                              "type" : "ArmyBookRule"
                           }
                        ],
                        "type" : "ArmyBookWeapon",
                        "weaponId" : "0-erIAQg"
                     },
                     {
                        "attacks" : 1,
                        "count" : 1,
                        "dependencies" : [],
                        "label" : "Sgt. Heavy Pistol (12\", A1, AP(1))",
                        "name" : "Sgt. Heavy Pistol",
                        "range" : 12,
                        "specialRules" : [
                           {
                              "key" : "ap",
                              "label" : "AP(1)",
                              "modify" : false,
                              "name" : "AP",
                              "rating" : 1,
                              "type" : "ArmyBookRule"
                           }
                        ],
                        "type" : "ArmyBookWeapon",
                        "weaponId" : "XhE1E2aJ"
                     }
                  ],
                  "id" : "3pqIMyr",
                  "label" : "Energy Sword (A2, AP(1), Rending), Sgt. Heavy Pistol (12\", A1, AP(1))",
                  "parentPackageUid" : "J1",
                  "parentSectionUid" : "H1Ka5oX",
                  "proposedCostHint" : [
                     {
                        "isValid" : true,
                        "newCostPrecise" : "2.4",
                        "newCostRounded" : 0,
                        "unitName" : "Pistoleer Sisters"
                     }
                  ],
                  "proposedVersion" : "2.50",
                  "type" : "ArmyBookUpgradeOption",
                  "uid" : "3pqIMyr"
               },
               "upgrade" : {
                  "affects" : {
                     "type" : "exactly",
                     "value" : 1
                  },
                  "id" : "ObjDZeD",
                  "label" : "Replace one Twin Heavy Pistols and CCW",
                  "options" : null,
                  "parentPackageUid" : "J1",
                  "targets" : [
                     "Twin Heavy Pistols",
                     "CCW"
                  ],
                  "type" : "ArmyBookUpgradeSection",
                  "uid" : "H1Ka5oX",
                  "variant" : "replace"
               }
            }
         ],
         "selectionId" : "E2Xrs",
         "size" : 5,
         "specialRules" : [
            {
               "key" : "ambush",
               "name" : "Ambush",
               "rating" : ""
            },
            {
               "additional" : false,
               "key" : "devout",
               "name" : "Devout",
               "rating" : ""
            },
            {
               "key" : "flying",
               "name" : "Flying",
               "rating" : ""
            }
         ],
         "traits" : [],
         "upgrades" : [
            "J1"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "7oi8zeiqfamiur21",
         "combined" : false,
         "cost" : 110,
         "defense" : 4,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 1,
               "count" : 5,
               "id" : "WWqUi2Wx",
               "label" : "CCW (A1)",
               "name" : "CCW",
               "originalCount" : 5,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "4o5CBBZC"
            },
            {
               "attacks" : 1,
               "count" : 5,
               "id" : "lXOrCYv-",
               "label" : "Heavy Rifle (24\", A1, AP(1))",
               "name" : "Heavy Rifle",
               "originalCount" : 5,
               "range" : 24,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "lXOrCYv-"
            }
         ],
         "id" : "0Bc4px2",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 1,
               "count" : 5,
               "id" : "WWqUi2Wx",
               "label" : "CCW (A1)",
               "name" : "CCW",
               "originalCount" : 5,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "4o5CBBZC"
            },
            {
               "attacks" : 1,
               "count" : 5,
               "id" : "lXOrCYv-",
               "label" : "Heavy Rifle (24\", A1, AP(1))",
               "name" : "Heavy Rifle",
               "originalCount" : 5,
               "range" : 24,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "lXOrCYv-"
            }
         ],
         "name" : "Warrior Sisters",
         "notes" : null,
         "quality" : 4,
         "selectedUpgrades" : [],
         "selectionId" : "qu16L",
         "size" : 5,
         "specialRules" : [
            {
               "additional" : false,
               "key" : "devout",
               "name" : "Devout",
               "rating" : ""
            }
         ],
         "traits" : [],
         "upgrades" : [
            "E1",
            "cT2sU",
            "vo3TX"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "7oi8zeiqfamiur21",
         "combined" : false,
         "cost" : 85,
         "customName" : "Arco flagellant",
         "defense" : 5,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : "2",
               "count" : 5,
               "dependencies" : [
                  {
                     "count" : 5,
                     "upgradeInstanceId" : "Yr64lLfjt",
                     "variant" : "replace"
                  }
               ],
               "id" : "4MIc_Zub",
               "label" : "Chainsaw Sword (A2, AP(1))",
               "name" : "Chainsaw Sword",
               "originalCount" : 5,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "CClql4IC"
            }
         ],
         "id" : "R3sC7UX",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 2,
               "count" : 5,
               "dependencies" : [],
               "id" : "MKdDx8aj",
               "label" : "Dual Arc-Flails (A2, Rending)",
               "name" : "Dual Arc-Flails",
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "label" : "Rending",
                     "modify" : false,
                     "name" : "Rending",
                     "type" : "ArmyBookRule"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "MKdDx8aj"
            },
            {
               "content" : [
                  {
                     "count" : 5,
                     "dependencies" : [],
                     "key" : "regeneration",
                     "label" : "Regeneration",
                     "modify" : false,
                     "name" : "Regeneration",
                     "rating" : "",
                     "type" : "ArmyBookRule"
                  }
               ],
               "count" : 5,
               "dependencies" : [],
               "label" : "Flagellants (Regeneration)",
               "name" : "Flagellants",
               "type" : "ArmyBookItem"
            }
         ],
         "name" : "Fanatic Sisters",
         "notes" : null,
         "quality" : 5,
         "selectedUpgrades" : [
            {
               "instanceId" : "Yr64lLfjt",
               "option" : {
                  "cost" : -5,
                  "costs" : [
                     {
                        "cost" : -5,
                        "unitId" : "R3sC7UX"
                     }
                  ],
                  "gains" : [
                     {
                        "attacks" : 2,
                        "count" : 5,
                        "dependencies" : [],
                        "id" : "MKdDx8aj",
                        "label" : "Dual Arc-Flails (A2, Rending)",
                        "name" : "Dual Arc-Flails",
                        "range" : 0,
                        "specialRules" : [
                           {
                              "key" : "rending",
                              "label" : "Rending",
                              "modify" : false,
                              "name" : "Rending",
                              "type" : "ArmyBookRule"
                           }
                        ],
                        "type" : "ArmyBookWeapon",
                        "weaponId" : "MKdDx8aj"
                     }
                  ],
                  "id" : "AwpWO",
                  "label" : "Dual Arc-Flails (A2, Rending)",
                  "parentPackageUid" : "H1",
                  "parentSectionUid" : "kNZAW",
                  "proposedCost" : 25,
                  "proposedCostHint" : [
                     {
                        "isValid" : true,
                        "newCostPrecise" : "-6.7",
                        "newCostRounded" : -5,
                        "unitName" : "Fanatic Sisters"
                     }
                  ],
                  "proposedVersion" : "2.50",
                  "type" : "ArmyBookUpgradeOption",
                  "uid" : "AwpWO"
               },
               "upgrade" : {
                  "affects" : {
                     "type" : "all"
                  },
                  "id" : "TqUDp7l",
                  "label" : "Replace all Chainsaw Swords",
                  "options" : null,
                  "parentPackageUid" : "H1",
                  "targets" : [
                     "Chainsaw Swords"
                  ],
                  "type" : "ArmyBookUpgradeSection",
                  "uid" : "kNZAW",
                  "variant" : "replace"
               }
            },
            {
               "instanceId" : "4iYNtvc67",
               "option" : {
                  "cost" : 15,
                  "costs" : [
                     {
                        "cost" : 15,
                        "unitId" : "R3sC7UX"
                     }
                  ],
                  "gains" : [
                     {
                        "content" : [
                           {
                              "count" : 5,
                              "dependencies" : [],
                              "key" : "regeneration",
                              "label" : "Regeneration",
                              "modify" : false,
                              "name" : "Regeneration",
                              "rating" : "",
                              "type" : "ArmyBookRule"
                           }
                        ],
                        "count" : 5,
                        "dependencies" : [],
                        "label" : "Flagellants (Regeneration)",
                        "name" : "Flagellants",
                        "type" : "ArmyBookItem"
                     }
                  ],
                  "id" : "q4Db3",
                  "label" : "Flagellants (Regeneration)",
                  "parentPackageUid" : "H1",
                  "parentSectionUid" : "Zl_L3",
                  "proposedCost" : 10,
                  "proposedCostHint" : [
                     {
                        "isValid" : true,
                        "newCostPrecise" : "13.3",
                        "newCostRounded" : 15,
                        "unitName" : "Fanatic Sisters"
                     }
                  ],
                  "proposedVersion" : "2.50",
                  "type" : "ArmyBookUpgradeOption",
                  "uid" : "q4Db3"
               },
               "upgrade" : {
                  "affects" : {
                     "type" : "all"
                  },
                  "id" : "UiSJSad",
                  "label" : "Upgrade all models with",
                  "model" : true,
                  "options" : null,
                  "parentPackageUid" : "H1",
                  "type" : "ArmyBookUpgradeSection",
                  "uid" : "Zl_L3",
                  "variant" : "upgrade"
               }
            }
         ],
         "selectionId" : "Bpvw_",
         "size" : 5,
         "specialRules" : [
            {
               "key" : "fearless",
               "name" : "Fearless",
               "rating" : ""
            },
            {
               "id" : 212,
               "key" : "frenzy",
               "name" : "Frenzy"
            }
         ],
         "traits" : [],
         "upgrades" : [
            "H1"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "7oi8zeiqfamiur21",
         "combined" : false,
         "cost" : 265,
         "defense" : 2,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 3,
               "count" : 1,
               "id" : "ywxRIDQs",
               "label" : "Stomp (A3, AP(1))",
               "name" : "Stomp",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "-iQKsJot"
            },
            {
               "attacks" : 1,
               "count" : 2,
               "id" : "mdn1RlHy",
               "label" : "Heavy Flail (A1, AP(1), Blast(3))",
               "name" : "Heavy Flail",
               "originalCount" : 2,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  },
                  {
                     "key" : "blast",
                     "modify" : false,
                     "name" : "Blast",
                     "rating" : "3"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "mdn1RlHy"
            }
         ],
         "id" : "2K2sl7L",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 3,
               "count" : 1,
               "id" : "ywxRIDQs",
               "label" : "Stomp (A3, AP(1))",
               "name" : "Stomp",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "-iQKsJot"
            },
            {
               "attacks" : 1,
               "count" : 2,
               "id" : "mdn1RlHy",
               "label" : "Heavy Flail (A1, AP(1), Blast(3))",
               "name" : "Heavy Flail",
               "originalCount" : 2,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  },
                  {
                     "key" : "blast",
                     "modify" : false,
                     "name" : "Blast",
                     "rating" : "3"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "mdn1RlHy"
            }
         ],
         "name" : "Assault Walker",
         "notes" : null,
         "quality" : 4,
         "selectedUpgrades" : [],
         "selectionId" : "oNDoX",
         "size" : 1,
         "specialRules" : [
            {
               "additional" : false,
               "key" : "devout",
               "name" : "Devout",
               "rating" : ""
            },
            {
               "key" : "fear",
               "label" : "Fear(2)",
               "name" : "Fear",
               "rating" : 2
            },
            {
               "key" : "fearless",
               "name" : "Fearless",
               "rating" : ""
            },
            {
               "key" : "furious",
               "name" : "Furious",
               "rating" : ""
            },
            {
               "key" : "regeneration",
               "name" : "Regeneration",
               "rating" : ""
            },
            {
               "key" : "tough",
               "name" : "Tough",
               "rating" : "9"
            }
         ],
         "traits" : [],
         "upgrades" : [
            "viyjq",
            "ObQny"
         ],
         "valid" : true,
         "xp" : 0
      }
   ]
}
"#,
    // https://army-forge.onepagerules.com/share?id=Mlwpoh1AGLC2&name=WH_Imperium%20-%20Necrons
    r#"
{
   "campaignMode" : false,
   "competitive" : true,
   "gameSystem" : "gf",
   "id" : "Mlwpoh1AGLC2",
   "isCloud" : false,
   "modified" : "2024-01-02T22:58:09.356Z",
   "name" : "WH Imperium - Necrons",
   "points" : 3255,
   "pointsLimit" : 4000,
   "selectedUnitId" : "nXvZ_",
   "specialRules" : [
      {
         "aliasedRuleId" : null,
         "description" : "Once per game, when this model is activated, you may place a new unit of 3 Bot Swarms fully within 6” of it.",
         "hasRating" : false,
         "id" : 266,
         "name" : "Release Swarm"
      },
      {
         "aliasedRuleId" : 162,
         "description" : "This model and its unit may ignore the Slow rule.",
         "hasRating" : false,
         "id" : 162,
         "name" : "Royal March"
      },
      {
         "aliasedRuleId" : 51,
         "description" : "This model and its unit get Ambush.",
         "hasRating" : false,
         "id" : 51,
         "name" : "Shadow-Protocol"
      },
      {
         "aliasedRuleId" : null,
         "description" : "This model and its unit get +1 to Regeneration rolls.",
         "hasRating" : false,
         "id" : 265,
         "name" : "Regen-Protocol"
      },
      {
         "aliasedRuleId" : null,
         "description" : "Once per activation, pick one friendly unit within 12”, which gets +1 to Regeneration rolls next time it takes wounds.",
         "hasRating" : false,
         "id" : 264,
         "name" : "Reanimator"
      },
      {
         "aliasedRuleId" : null,
         "description" : "Once per activation, pick one friendly Caster within 6”, which gets +1 to its next spell casting roll.",
         "hasRating" : false,
         "id" : 263,
         "name" : "Spell Warden"
      },
      {
         "aliasedRuleId" : null,
         "description" : "Once per activation, pick one friendly unit within 12”, which may ignore the Slow rule next time it moves.",
         "hasRating" : false,
         "id" : 269,
         "name" : "Royal March Order"
      },
      {
         "aliasedRuleId" : null,
         "description" : "This model counts as having Caster(6), but takes 3 wounds whenever it fails to cast a spell.",
         "hasRating" : false,
         "id" : 262,
         "name" : "Spell Master"
      },
      {
         "aliasedRuleId" : null,
         "description" : "Counts as having the Ambush rule, and gets AP(+3) when shooting on the round in which it deploys.",
         "hasRating" : false,
         "id" : 261,
         "name" : "Hunter"
      },
      {
         "aliasedRuleId" : null,
         "description" : "This model gets +1 to defense rolls against non-spell attacks.",
         "hasRating" : false,
         "id" : 858,
         "name" : "Shield Wall"
      },
      {
         "aliasedRuleId" : null,
         "description" : "Once per activation, if within 2” of a model with Tough, roll one die. On a 2+ you may remove D3 wounds from that model.",
         "hasRating" : false,
         "id" : 126,
         "name" : "Repair"
      },
      {
         "aliasedRuleId" : null,
         "description" : "Enemy units can’t be set up within 12” of this model when using Ambush.",
         "hasRating" : false,
         "id" : 370,
         "name" : "Warning Cry"
      },
      {
         "aliasedRuleId" : null,
         "description" : "This model counts as having Ambush, and may be deployed up to 1” away from enemy units.",
         "hasRating" : false,
         "id" : 166,
         "name" : "Tunneller"
      },
      {
         "aliasedRuleId" : 149,
         "description" : "When this model and its unit take a wound, roll one die, and on a 6+ it is ignored. If the wound was from a spell, then it is ignored on a 4+ instead.",
         "hasRating" : false,
         "id" : 149,
         "name" : "Gloom-Protocol"
      },
      {
         "aliasedRuleId" : 216,
         "description" : "Whenever this unit fails a morale test, it counts as passed instead. Then, roll as many dice as remaining models/tough with this rule, and for each result of 1-3 the unit takes one wound, which can’t be ignored.",
         "hasRating" : false,
         "id" : 216,
         "name" : "Robot"
      },
      {
         "aliasedRuleId" : null,
         "description" : "Unmodified rolls of 6 are multiplied by 2 (only the original hit counts as a 6).",
         "hasRating" : false,
         "id" : 846,
         "name" : "Flux"
      }
   ],
   "undoUnitRemove" : [
      {
         "armyId" : "4k5amkxoybdiqotm",
         "combined" : false,
         "cost" : 125,
         "defense" : 4,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 1,
               "count" : 5,
               "id" : "_WpwGRUk",
               "label" : "CCW (A1)",
               "name" : "CCW",
               "originalCount" : 5,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "4o5CBBZC"
            },
            {
               "attacks" : 1,
               "count" : 5,
               "dependencies" : [
                  {
                     "count" : 5,
                     "upgradeInstanceId" : "-cmtbU9na",
                     "variant" : "replace"
                  }
               ],
               "id" : "OaYz4pdg",
               "label" : "Gauss Rifle (24\", A1, Rending)",
               "name" : "Gauss Rifle",
               "originalCount" : 5,
               "range" : 24,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "OaYz4pdg"
            }
         ],
         "id" : "njiIkJi",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 1,
               "count" : 5,
               "id" : "_WpwGRUk",
               "label" : "CCW (A1)",
               "name" : "CCW",
               "originalCount" : 5,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "4o5CBBZC"
            },
            {
               "attacks" : 1,
               "count" : 5,
               "dependencies" : [],
               "id" : "YsdNdfjc",
               "label" : "Reaper Rifle (18\", A1, AP(2))",
               "name" : "Reaper Rifle",
               "range" : 18,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "label" : "AP(2)",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "2",
                     "type" : "ArmyBookRule"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "YsdNdfjc"
            }
         ],
         "name" : "Warriors",
         "notes" : null,
         "quality" : 3,
         "selectedUpgrades" : [
            {
               "instanceId" : "-cmtbU9na",
               "option" : {
                  "cost" : 5,
                  "costs" : [
                     {
                        "cost" : 5,
                        "unitId" : "njiIkJi"
                     }
                  ],
                  "gains" : [
                     {
                        "attacks" : 1,
                        "count" : 5,
                        "dependencies" : [],
                        "id" : "YsdNdfjc",
                        "label" : "Reaper Rifle (18\", A1, AP(2))",
                        "name" : "Reaper Rifle",
                        "range" : 18,
                        "specialRules" : [
                           {
                              "key" : "ap",
                              "label" : "AP(2)",
                              "modify" : false,
                              "name" : "AP",
                              "rating" : "2",
                              "type" : "ArmyBookRule"
                           }
                        ],
                        "type" : "ArmyBookWeapon",
                        "weaponId" : "YsdNdfjc"
                     }
                  ],
                  "id" : "r_yHp",
                  "label" : "Reaper Rifle (18\", A1, AP(2))",
                  "parentSectionId" : "sPruWe5",
                  "type" : "ArmyBookUpgradeOption",
                  "uid" : "r_yHp"
               },
               "upgrade" : {
                  "affects" : {
                     "type" : "all"
                  },
                  "id" : "X6U6DMX",
                  "isCommandGroup" : false,
                  "label" : "Replace all Gauss Rifles",
                  "options" : null,
                  "parentPackageUid" : "E1",
                  "targets" : [
                     "Gauss Rifles"
                  ],
                  "type" : "ArmyBookUpgradeSection",
                  "uid" : "sPruWe5",
                  "variant" : "replace"
               }
            }
         ],
         "selectionId" : "cn9e9",
         "size" : 5,
         "sortId" : 3,
         "specialRules" : [
            {
               "key" : "regeneration",
               "name" : "Regeneration",
               "rating" : ""
            },
            {
               "key" : "robot",
               "name" : "Robot",
               "rating" : ""
            },
            {
               "key" : "slow",
               "name" : "Slow",
               "rating" : ""
            }
         ],
         "traits" : [],
         "upgrades" : [
            "E1"
         ],
         "valid" : true,
         "xp" : 0
      }
   ],
   "unitPreview" : null,
   "units" : [
      {
         "armyId" : "4k5amkxoybdiqotm",
         "combined" : false,
         "cost" : 65,
         "customName" : "Overlord",
         "defense" : 3,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 3,
               "count" : 1,
               "id" : "0L3LR8_n",
               "label" : "CCW (A3)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "tO5jEqMO"
            },
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "_g6MjuFl",
               "label" : "Lord Gauss Pistol (12\", A2, Rending)",
               "name" : "Lord Gauss Pistol",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "_g6MjuFl"
            }
         ],
         "id" : "lcEWPMS",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 3,
               "count" : 1,
               "id" : "0L3LR8_n",
               "label" : "CCW (A3)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "tO5jEqMO"
            },
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "_g6MjuFl",
               "label" : "Lord Gauss Pistol (12\", A2, Rending)",
               "name" : "Lord Gauss Pistol",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "_g6MjuFl"
            },
            {
               "content" : [
                  {
                     "count" : 1,
                     "dependencies" : [],
                     "key" : "royal-march",
                     "label" : "Royal March",
                     "name" : "Royal March",
                     "rating" : "",
                     "type" : "ArmyBookRule"
                  }
               ],
               "count" : 1,
               "dependencies" : [],
               "label" : "Overseer (Royal March)",
               "name" : "Overseer",
               "type" : "ArmyBookItem"
            }
         ],
         "name" : "Robot Lord",
         "notes" : null,
         "quality" : 3,
         "selectedUpgrades" : [
            {
               "instanceId" : "iLgixc_cZ",
               "option" : {
                  "cost" : 25,
                  "costs" : [
                     {
                        "cost" : 25,
                        "unitId" : "lcEWPMS"
                     },
                     {
                        "cost" : 25,
                        "unitId" : "y8xIlHV"
                     },
                     {
                        "cost" : 25,
                        "unitId" : "R2d8LeD"
                     }
                  ],
                  "gains" : [
                     {
                        "content" : [
                           {
                              "count" : 1,
                              "dependencies" : [],
                              "key" : "royal-march",
                              "label" : "Royal March",
                              "name" : "Royal March",
                              "rating" : "",
                              "type" : "ArmyBookRule"
                           }
                        ],
                        "count" : 1,
                        "dependencies" : [],
                        "label" : "Overseer (Royal March)",
                        "name" : "Overseer",
                        "type" : "ArmyBookItem"
                     }
                  ],
                  "id" : "AZDWCqf",
                  "label" : "Overseer (Royal March)",
                  "parentPackageUid" : "C1",
                  "parentSectionUid" : "IxcS2nP",
                  "proposedCost" : 25,
                  "proposedCostHint" : [
                     {
                        "isValid" : true,
                        "newCostPrecise" : "24.0",
                        "newCostRounded" : 25,
                        "unitName" : "Robot Lord"
                     },
                     {
                        "isValid" : true,
                        "newCostPrecise" : "24.0",
                        "newCostRounded" : 25,
                        "unitName" : "Annihilator Lord"
                     },
                     {
                        "isValid" : true,
                        "newCostPrecise" : "24.0",
                        "newCostRounded" : 25,
                        "unitName" : "Tri-Scorpion Lord"
                     }
                  ],
                  "proposedVersion" : "2.50",
                  "type" : "ArmyBookUpgradeOption",
                  "uid" : "AZDWCqf"
               },
               "upgrade" : {
                  "id" : "C_Wx1mp",
                  "isHeroUpgrade" : true,
                  "label" : "Upgrade with one",
                  "options" : null,
                  "parentPackageUid" : "C1",
                  "select" : {
                     "type" : "exactly",
                     "value" : 1
                  },
                  "type" : "ArmyBookUpgradeSection",
                  "uid" : "IxcS2nP",
                  "variant" : "upgrade"
               }
            }
         ],
         "selectionId" : "BPkEu",
         "size" : 1,
         "specialRules" : [
            {
               "key" : "hero",
               "name" : "Hero",
               "rating" : ""
            },
            {
               "key" : "regeneration",
               "name" : "Regeneration",
               "rating" : ""
            },
            {
               "key" : "robot",
               "name" : "Robot",
               "rating" : ""
            },
            {
               "key" : "slow",
               "name" : "Slow",
               "rating" : ""
            },
            {
               "key" : "tough",
               "name" : "Tough",
               "rating" : "3"
            }
         ],
         "traits" : [],
         "upgrades" : [
            "A1",
            "B1",
            "C1"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "4k5amkxoybdiqotm",
         "combined" : false,
         "cost" : 65,
         "customName" : "Royal Warden",
         "defense" : 3,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 3,
               "count" : 1,
               "id" : "0L3LR8_n",
               "label" : "CCW (A3)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "tO5jEqMO"
            },
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "_g6MjuFl",
               "label" : "Lord Gauss Pistol (12\", A2, Rending)",
               "name" : "Lord Gauss Pistol",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "_g6MjuFl"
            }
         ],
         "id" : "lcEWPMS",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 3,
               "count" : 1,
               "id" : "0L3LR8_n",
               "label" : "CCW (A3)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "tO5jEqMO"
            },
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "_g6MjuFl",
               "label" : "Lord Gauss Pistol (12\", A2, Rending)",
               "name" : "Lord Gauss Pistol",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "_g6MjuFl"
            },
            {
               "content" : [
                  {
                     "count" : 1,
                     "dependencies" : [],
                     "key" : "regen-protocol",
                     "label" : "Regen-Protocol",
                     "name" : "Regen-Protocol",
                     "rating" : "",
                     "type" : "ArmyBookRule"
                  }
               ],
               "count" : 1,
               "dependencies" : [],
               "label" : "Warden (Regen-Protocol)",
               "name" : "Warden",
               "type" : "ArmyBookItem"
            }
         ],
         "name" : "Robot Lord",
         "notes" : null,
         "quality" : 3,
         "selectedUpgrades" : [
            {
               "instanceId" : "I--Deorz8",
               "option" : {
                  "cost" : 20,
                  "costs" : [
                     {
                        "cost" : 20,
                        "unitId" : "lcEWPMS"
                     },
                     {
                        "cost" : 20,
                        "unitId" : "y8xIlHV"
                     },
                     {
                        "cost" : 20,
                        "unitId" : "R2d8LeD"
                     }
                  ],
                  "gains" : [
                     {
                        "content" : [
                           {
                              "count" : 1,
                              "dependencies" : [],
                              "key" : "regen-protocol",
                              "label" : "Regen-Protocol",
                              "name" : "Regen-Protocol",
                              "rating" : "",
                              "type" : "ArmyBookRule"
                           }
                        ],
                        "count" : 1,
                        "dependencies" : [],
                        "label" : "Warden (Regen-Protocol)",
                        "name" : "Warden",
                        "type" : "ArmyBookItem"
                     }
                  ],
                  "id" : "a-G2qj8",
                  "label" : "Warden (Regen-Protocol)",
                  "parentPackageUid" : "C1",
                  "parentSectionUid" : "IxcS2nP",
                  "proposedCost" : 25,
                  "proposedCostHint" : [
                     {
                        "isValid" : true,
                        "newCostPrecise" : "21.1",
                        "newCostRounded" : 20,
                        "unitName" : "Robot Lord"
                     },
                     {
                        "isValid" : true,
                        "newCostPrecise" : "21.1",
                        "newCostRounded" : 20,
                        "unitName" : "Annihilator Lord"
                     },
                     {
                        "isValid" : true,
                        "newCostPrecise" : "21.1",
                        "newCostRounded" : 20,
                        "unitName" : "Tri-Scorpion Lord"
                     }
                  ],
                  "proposedVersion" : "2.50",
                  "type" : "ArmyBookUpgradeOption",
                  "uid" : "a-G2qj8"
               },
               "upgrade" : {
                  "id" : "C_Wx1mp",
                  "isHeroUpgrade" : true,
                  "label" : "Upgrade with one",
                  "options" : null,
                  "parentPackageUid" : "C1",
                  "select" : {
                     "type" : "exactly",
                     "value" : 1
                  },
                  "type" : "ArmyBookUpgradeSection",
                  "uid" : "IxcS2nP",
                  "variant" : "upgrade"
               }
            }
         ],
         "selectionId" : "O5_9z",
         "size" : 1,
         "specialRules" : [
            {
               "key" : "hero",
               "name" : "Hero",
               "rating" : ""
            },
            {
               "key" : "regeneration",
               "name" : "Regeneration",
               "rating" : ""
            },
            {
               "key" : "robot",
               "name" : "Robot",
               "rating" : ""
            },
            {
               "key" : "slow",
               "name" : "Slow",
               "rating" : ""
            },
            {
               "key" : "tough",
               "name" : "Tough",
               "rating" : "3"
            }
         ],
         "traits" : [],
         "upgrades" : [
            "A1",
            "B1",
            "C1"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "4k5amkxoybdiqotm",
         "combined" : false,
         "cost" : 65,
         "customName" : "Technomancer",
         "defense" : 3,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 3,
               "count" : 1,
               "id" : "0L3LR8_n",
               "label" : "CCW (A3)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "tO5jEqMO"
            },
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "_g6MjuFl",
               "label" : "Lord Gauss Pistol (12\", A2, Rending)",
               "name" : "Lord Gauss Pistol",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "_g6MjuFl"
            }
         ],
         "id" : "lcEWPMS",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 3,
               "count" : 1,
               "id" : "0L3LR8_n",
               "label" : "CCW (A3)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "tO5jEqMO"
            },
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "_g6MjuFl",
               "label" : "Lord Gauss Pistol (12\", A2, Rending)",
               "name" : "Lord Gauss Pistol",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "_g6MjuFl"
            },
            {
               "content" : [
                  {
                     "count" : 1,
                     "dependencies" : [],
                     "key" : "caster",
                     "label" : "Caster(2)",
                     "name" : "Caster",
                     "rating" : 2,
                     "type" : "ArmyBookRule"
                  }
               ],
               "count" : 1,
               "dependencies" : [],
               "label" : "Technomancer (Caster(2))",
               "name" : "Technomancer",
               "type" : "ArmyBookItem"
            }
         ],
         "name" : "Robot Lord",
         "notes" : null,
         "quality" : 3,
         "selectedUpgrades" : [
            {
               "instanceId" : "Wql6Q1Eqp",
               "option" : {
                  "cost" : 40,
                  "costs" : [
                     {
                        "cost" : 40,
                        "unitId" : "lcEWPMS"
                     },
                     {
                        "cost" : 40,
                        "unitId" : "y8xIlHV"
                     },
                     {
                        "cost" : 40,
                        "unitId" : "R2d8LeD"
                     }
                  ],
                  "gains" : [
                     {
                        "content" : [
                           {
                              "count" : 1,
                              "dependencies" : [],
                              "key" : "caster",
                              "label" : "Caster(2)",
                              "name" : "Caster",
                              "rating" : 2,
                              "type" : "ArmyBookRule"
                           }
                        ],
                        "count" : 1,
                        "dependencies" : [],
                        "label" : "Technomancer (Caster(2))",
                        "name" : "Technomancer",
                        "type" : "ArmyBookItem"
                     }
                  ],
                  "id" : "pAgesdx",
                  "label" : "Technomancer (Caster(2))",
                  "parentPackageUid" : "C1",
                  "parentSectionUid" : "IxcS2nP",
                  "proposedCost" : 20,
                  "proposedCostHint" : [
                     {
                        "isValid" : true,
                        "newCostPrecise" : "40.0",
                        "newCostRounded" : 40,
                        "unitName" : "Robot Lord"
                     },
                     {
                        "isValid" : true,
                        "newCostPrecise" : "40.0",
                        "newCostRounded" : 40,
                        "unitName" : "Annihilator Lord"
                     },
                     {
                        "isValid" : true,
                        "newCostPrecise" : "40.0",
                        "newCostRounded" : 40,
                        "unitName" : "Tri-Scorpion Lord"
                     }
                  ],
                  "proposedVersion" : "2.50",
                  "type" : "ArmyBookUpgradeOption",
                  "uid" : "pAgesdx"
               },
               "upgrade" : {
                  "id" : "C_Wx1mp",
                  "isHeroUpgrade" : true,
                  "label" : "Upgrade with one",
                  "options" : null,
                  "parentPackageUid" : "C1",
                  "select" : {
                     "type" : "exactly",
                     "value" : 1
                  },
                  "type" : "ArmyBookUpgradeSection",
                  "uid" : "IxcS2nP",
                  "variant" : "upgrade"
               }
            }
         ],
         "selectionId" : "Zha0V",
         "size" : 1,
         "specialRules" : [
            {
               "key" : "hero",
               "name" : "Hero",
               "rating" : ""
            },
            {
               "key" : "regeneration",
               "name" : "Regeneration",
               "rating" : ""
            },
            {
               "key" : "robot",
               "name" : "Robot",
               "rating" : ""
            },
            {
               "key" : "slow",
               "name" : "Slow",
               "rating" : ""
            },
            {
               "key" : "tough",
               "name" : "Tough",
               "rating" : "3"
            }
         ],
         "traits" : [],
         "upgrades" : [
            "A1",
            "B1",
            "C1"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "4k5amkxoybdiqotm",
         "combined" : true,
         "cost" : 125,
         "defense" : 4,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 1,
               "count" : 5,
               "id" : "_WpwGRUk",
               "label" : "CCW (A1)",
               "name" : "CCW",
               "originalCount" : 5,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "4o5CBBZC"
            },
            {
               "attacks" : 1,
               "count" : 5,
               "id" : "OaYz4pdg",
               "label" : "Gauss Rifle (24\", A1, Rending)",
               "name" : "Gauss Rifle",
               "originalCount" : 5,
               "range" : 24,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "OaYz4pdg"
            }
         ],
         "id" : "njiIkJi",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 1,
               "count" : 5,
               "id" : "_WpwGRUk",
               "label" : "CCW (A1)",
               "name" : "CCW",
               "originalCount" : 5,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "4o5CBBZC"
            },
            {
               "attacks" : 1,
               "count" : 5,
               "id" : "OaYz4pdg",
               "label" : "Gauss Rifle (24\", A1, Rending)",
               "name" : "Gauss Rifle",
               "originalCount" : 5,
               "range" : 24,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "OaYz4pdg"
            }
         ],
         "name" : "Warriors",
         "notes" : null,
         "quality" : 3,
         "selectedUpgrades" : [],
         "selectionId" : "C7ByT",
         "size" : 5,
         "specialRules" : [
            {
               "key" : "regeneration",
               "name" : "Regeneration",
               "rating" : ""
            },
            {
               "key" : "robot",
               "name" : "Robot",
               "rating" : ""
            },
            {
               "key" : "slow",
               "name" : "Slow",
               "rating" : ""
            }
         ],
         "traits" : [],
         "upgrades" : [
            "E1"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "4k5amkxoybdiqotm",
         "combined" : true,
         "cost" : 125,
         "defense" : 4,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 1,
               "count" : 5,
               "id" : "_WpwGRUk",
               "label" : "CCW (A1)",
               "name" : "CCW",
               "originalCount" : 5,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "4o5CBBZC"
            },
            {
               "attacks" : 1,
               "count" : 5,
               "id" : "OaYz4pdg",
               "label" : "Gauss Rifle (24\", A1, Rending)",
               "name" : "Gauss Rifle",
               "originalCount" : 5,
               "range" : 24,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "OaYz4pdg"
            }
         ],
         "id" : "njiIkJi",
         "joinToUnit" : "C7ByT",
         "loadout" : [
            {
               "attacks" : 1,
               "count" : 5,
               "id" : "_WpwGRUk",
               "label" : "CCW (A1)",
               "name" : "CCW",
               "originalCount" : 5,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "4o5CBBZC"
            },
            {
               "attacks" : 1,
               "count" : 5,
               "id" : "OaYz4pdg",
               "label" : "Gauss Rifle (24\", A1, Rending)",
               "name" : "Gauss Rifle",
               "originalCount" : 5,
               "range" : 24,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "OaYz4pdg"
            }
         ],
         "name" : "Warriors",
         "notes" : null,
         "quality" : 3,
         "selectedUpgrades" : [],
         "selectionId" : "No3Z8",
         "size" : 5,
         "specialRules" : [
            {
               "key" : "regeneration",
               "name" : "Regeneration",
               "rating" : ""
            },
            {
               "key" : "robot",
               "name" : "Robot",
               "rating" : ""
            },
            {
               "key" : "slow",
               "name" : "Slow",
               "rating" : ""
            }
         ],
         "traits" : [],
         "upgrades" : [
            "E1"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "4k5amkxoybdiqotm",
         "combined" : true,
         "cost" : 125,
         "defense" : 4,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 1,
               "count" : 5,
               "id" : "_WpwGRUk",
               "label" : "CCW (A1)",
               "name" : "CCW",
               "originalCount" : 5,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "4o5CBBZC"
            },
            {
               "attacks" : 1,
               "count" : 5,
               "dependencies" : [
                  {
                     "count" : 5,
                     "upgradeInstanceId" : "WtFEexy_N",
                     "variant" : "replace"
                  }
               ],
               "id" : "OaYz4pdg",
               "label" : "Gauss Rifle (24\", A1, Rending)",
               "name" : "Gauss Rifle",
               "originalCount" : 5,
               "range" : 24,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "OaYz4pdg"
            }
         ],
         "id" : "njiIkJi",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 1,
               "count" : 5,
               "id" : "_WpwGRUk",
               "label" : "CCW (A1)",
               "name" : "CCW",
               "originalCount" : 5,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "4o5CBBZC"
            },
            {
               "attacks" : 1,
               "count" : 5,
               "dependencies" : [],
               "id" : "YsdNdfjc",
               "label" : "Reaper Rifle (18\", A1, AP(2))",
               "name" : "Reaper Rifle",
               "range" : 18,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "label" : "AP(2)",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "2",
                     "type" : "ArmyBookRule"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "YsdNdfjc"
            }
         ],
         "name" : "Warriors",
         "notes" : null,
         "quality" : 3,
         "selectedUpgrades" : [
            {
               "instanceId" : "WtFEexy_N",
               "option" : {
                  "cost" : 5,
                  "costs" : [
                     {
                        "cost" : 5,
                        "unitId" : "njiIkJi"
                     }
                  ],
                  "gains" : [
                     {
                        "attacks" : 1,
                        "count" : 5,
                        "dependencies" : [],
                        "id" : "YsdNdfjc",
                        "label" : "Reaper Rifle (18\", A1, AP(2))",
                        "name" : "Reaper Rifle",
                        "range" : 18,
                        "specialRules" : [
                           {
                              "key" : "ap",
                              "label" : "AP(2)",
                              "modify" : false,
                              "name" : "AP",
                              "rating" : "2",
                              "type" : "ArmyBookRule"
                           }
                        ],
                        "type" : "ArmyBookWeapon",
                        "weaponId" : "YsdNdfjc"
                     }
                  ],
                  "id" : "r_yHp",
                  "label" : "Reaper Rifle (18\", A1, AP(2))",
                  "parentPackageUid" : "E1",
                  "parentSectionUid" : "sPruWe5",
                  "proposedCost" : -5,
                  "proposedCostHint" : [
                     {
                        "isValid" : true,
                        "newCostPrecise" : "5.4",
                        "newCostRounded" : 5,
                        "unitName" : "Warriors"
                     }
                  ],
                  "proposedVersion" : "2.50",
                  "type" : "ArmyBookUpgradeOption",
                  "uid" : "r_yHp"
               },
               "upgrade" : {
                  "affects" : {
                     "type" : "all"
                  },
                  "id" : "N14hw8F",
                  "label" : "Replace all Gauss Rifles",
                  "options" : null,
                  "parentPackageUid" : "E1",
                  "targets" : [
                     "Gauss Rifles"
                  ],
                  "type" : "ArmyBookUpgradeSection",
                  "uid" : "sPruWe5",
                  "variant" : "replace"
               }
            }
         ],
         "selectionId" : "-TBrs",
         "size" : 5,
         "specialRules" : [
            {
               "key" : "regeneration",
               "name" : "Regeneration",
               "rating" : ""
            },
            {
               "key" : "robot",
               "name" : "Robot",
               "rating" : ""
            },
            {
               "key" : "slow",
               "name" : "Slow",
               "rating" : ""
            }
         ],
         "traits" : [],
         "upgrades" : [
            "E1"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "4k5amkxoybdiqotm",
         "combined" : true,
         "cost" : 125,
         "defense" : 4,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 1,
               "count" : 5,
               "id" : "_WpwGRUk",
               "label" : "CCW (A1)",
               "name" : "CCW",
               "originalCount" : 5,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "4o5CBBZC"
            },
            {
               "attacks" : 1,
               "count" : 5,
               "dependencies" : [
                  {
                     "count" : 5,
                     "upgradeInstanceId" : "XSA8B6nQM",
                     "variant" : "replace"
                  }
               ],
               "id" : "OaYz4pdg",
               "label" : "Gauss Rifle (24\", A1, Rending)",
               "name" : "Gauss Rifle",
               "originalCount" : 5,
               "range" : 24,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "OaYz4pdg"
            }
         ],
         "id" : "njiIkJi",
         "joinToUnit" : "-TBrs",
         "loadout" : [
            {
               "attacks" : 1,
               "count" : 5,
               "id" : "_WpwGRUk",
               "label" : "CCW (A1)",
               "name" : "CCW",
               "originalCount" : 5,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "4o5CBBZC"
            },
            {
               "attacks" : 1,
               "count" : 5,
               "dependencies" : [],
               "id" : "YsdNdfjc",
               "label" : "Reaper Rifle (18\", A1, AP(2))",
               "name" : "Reaper Rifle",
               "range" : 18,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "label" : "AP(2)",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "2",
                     "type" : "ArmyBookRule"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "YsdNdfjc"
            }
         ],
         "name" : "Warriors",
         "notes" : null,
         "quality" : 3,
         "selectedUpgrades" : [
            {
               "instanceId" : "XSA8B6nQM",
               "option" : {
                  "cost" : 5,
                  "costs" : [
                     {
                        "cost" : 5,
                        "unitId" : "njiIkJi"
                     }
                  ],
                  "gains" : [
                     {
                        "attacks" : 1,
                        "count" : 5,
                        "dependencies" : [],
                        "id" : "YsdNdfjc",
                        "label" : "Reaper Rifle (18\", A1, AP(2))",
                        "name" : "Reaper Rifle",
                        "range" : 18,
                        "specialRules" : [
                           {
                              "key" : "ap",
                              "label" : "AP(2)",
                              "modify" : false,
                              "name" : "AP",
                              "rating" : "2",
                              "type" : "ArmyBookRule"
                           }
                        ],
                        "type" : "ArmyBookWeapon",
                        "weaponId" : "YsdNdfjc"
                     }
                  ],
                  "id" : "r_yHp",
                  "label" : "Reaper Rifle (18\", A1, AP(2))",
                  "parentPackageUid" : "E1",
                  "parentSectionUid" : "sPruWe5",
                  "proposedCost" : -5,
                  "proposedCostHint" : [
                     {
                        "isValid" : true,
                        "newCostPrecise" : "5.4",
                        "newCostRounded" : 5,
                        "unitName" : "Warriors"
                     }
                  ],
                  "proposedVersion" : "2.50",
                  "type" : "ArmyBookUpgradeOption",
                  "uid" : "r_yHp"
               },
               "upgrade" : {
                  "affects" : {
                     "type" : "all"
                  },
                  "id" : "N14hw8F",
                  "label" : "Replace all Gauss Rifles",
                  "options" : null,
                  "parentPackageUid" : "E1",
                  "targets" : [
                     "Gauss Rifles"
                  ],
                  "type" : "ArmyBookUpgradeSection",
                  "uid" : "sPruWe5",
                  "variant" : "replace"
               }
            }
         ],
         "selectionId" : "5mNwx",
         "size" : 5,
         "specialRules" : [
            {
               "key" : "regeneration",
               "name" : "Regeneration",
               "rating" : ""
            },
            {
               "key" : "robot",
               "name" : "Robot",
               "rating" : ""
            },
            {
               "key" : "slow",
               "name" : "Slow",
               "rating" : ""
            }
         ],
         "traits" : [],
         "upgrades" : [
            "E1"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "4k5amkxoybdiqotm",
         "combined" : false,
         "cost" : 140,
         "defense" : 3,
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 2,
               "count" : 5,
               "id" : "Q47Qlon8",
               "label" : "Metal Claws (A2, AP(1))",
               "name" : "Metal Claws",
               "originalCount" : 5,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "Q47Qlon8"
            }
         ],
         "id" : "6q8HvOZ",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 2,
               "count" : 5,
               "id" : "Q47Qlon8",
               "label" : "Metal Claws (A2, AP(1))",
               "name" : "Metal Claws",
               "originalCount" : 5,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "Q47Qlon8"
            }
         ],
         "name" : "Flesh-Eaters",
         "notes" : null,
         "quality" : 3,
         "selectedUpgrades" : [],
         "selectionId" : "LP78D",
         "size" : 5,
         "specialRules" : [
            {
               "key" : "regeneration",
               "name" : "Regeneration",
               "rating" : ""
            },
            {
               "key" : "robot",
               "name" : "Robot",
               "rating" : ""
            },
            {
               "key" : "slow",
               "name" : "Slow",
               "rating" : ""
            },
            {
               "key" : "tunneller",
               "name" : "Tunneller",
               "rating" : ""
            }
         ],
         "traits" : [],
         "upgrades" : [],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "4k5amkxoybdiqotm",
         "combined" : false,
         "cost" : 75,
         "defense" : 6,
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 3,
               "count" : 3,
               "id" : "DvYpHLlf",
               "label" : "Swarm Attacks (A3, Rending)",
               "name" : "Swarm Attacks",
               "originalCount" : 3,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "bUQRotbI"
            }
         ],
         "id" : "yoeQi4m",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 3,
               "count" : 3,
               "id" : "DvYpHLlf",
               "label" : "Swarm Attacks (A3, Rending)",
               "name" : "Swarm Attacks",
               "originalCount" : 3,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "bUQRotbI"
            }
         ],
         "name" : "Bot Swarms",
         "notes" : null,
         "quality" : 6,
         "selectedUpgrades" : [],
         "selectionId" : "HXX3-",
         "size" : 3,
         "specialRules" : [
            {
               "key" : "regeneration",
               "name" : "Regeneration",
               "rating" : ""
            },
            {
               "key" : "robot",
               "name" : "Robot",
               "rating" : ""
            },
            {
               "key" : "strider",
               "name" : "Strider",
               "rating" : ""
            },
            {
               "key" : "tough",
               "name" : "Tough",
               "rating" : "3"
            }
         ],
         "traits" : [],
         "upgrades" : [],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "4k5amkxoybdiqotm",
         "combined" : false,
         "cost" : 255,
         "customName" : "Skorpekh",
         "defense" : 3,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 4,
               "count" : 3,
               "dependencies" : [
                  {
                     "count" : 1,
                     "upgradeInstanceId" : "nV6CNzckH",
                     "variant" : "replace"
                  }
               ],
               "id" : "NkTvh2sj",
               "label" : "Dual Reaper Blades (A4, AP(2))",
               "name" : "Dual Reaper Blades",
               "originalCount" : 3,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "2"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "NkTvh2sj"
            }
         ],
         "id" : "XgJtpyh",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 4,
               "count" : 2,
               "dependencies" : [
                  {
                     "count" : 1,
                     "upgradeInstanceId" : "nV6CNzckH",
                     "variant" : "replace"
                  }
               ],
               "id" : "NkTvh2sj",
               "label" : "Dual Reaper Blades (A4, AP(2))",
               "name" : "Dual Reaper Blades",
               "originalCount" : 3,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "2"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "NkTvh2sj"
            },
            {
               "attacks" : 1,
               "count" : 1,
               "dependencies" : [],
               "id" : "re7afFw-",
               "label" : "Heavy Reaper Blade (A1, AP(4), Deadly(3))",
               "name" : "Heavy Reaper Blade",
               "range" : 0,
               "specialRules" : [
                  {
                     "label" : "AP(4)",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : 4,
                     "type" : "ArmyBookRule"
                  },
                  {
                     "label" : "Deadly(3)",
                     "modify" : false,
                     "name" : "Deadly",
                     "rating" : 3,
                     "type" : "ArmyBookRule"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "re7afFw-"
            },
            {
               "content" : [
                  {
                     "count" : 3,
                     "dependencies" : [],
                     "label" : "Rending in Melee",
                     "modify" : false,
                     "name" : "Rending in Melee",
                     "type" : "ArmyBookRule"
                  }
               ],
               "count" : 3,
               "dependencies" : [],
               "label" : "Plasmabot (Rending in Melee)",
               "name" : "Plasmabot",
               "type" : "ArmyBookItem"
            }
         ],
         "name" : "Tri-Scorpions",
         "notes" : null,
         "quality" : 3,
         "selectedUpgrades" : [
            {
               "instanceId" : "nV6CNzckH",
               "option" : {
                  "cost" : -10,
                  "costs" : [
                     {
                        "cost" : -10,
                        "unitId" : "gcoBL7U"
                     },
                     {
                        "cost" : -10,
                        "unitId" : "XgJtpyh"
                     }
                  ],
                  "gains" : [
                     {
                        "attacks" : 1,
                        "count" : 1,
                        "dependencies" : [],
                        "id" : "re7afFw-",
                        "label" : "Heavy Reaper Blade (A1, AP(4), Deadly(3))",
                        "name" : "Heavy Reaper Blade",
                        "range" : 0,
                        "specialRules" : [
                           {
                              "label" : "AP(4)",
                              "modify" : false,
                              "name" : "AP",
                              "rating" : 4,
                              "type" : "ArmyBookRule"
                           },
                           {
                              "label" : "Deadly(3)",
                              "modify" : false,
                              "name" : "Deadly",
                              "rating" : 3,
                              "type" : "ArmyBookRule"
                           }
                        ],
                        "type" : "ArmyBookWeapon",
                        "weaponId" : "re7afFw-"
                     }
                  ],
                  "id" : "eDUyrWE",
                  "label" : "Heavy Reaper Blade (A1, AP(4), Deadly(3))",
                  "parentPackageUid" : "K1",
                  "parentSectionUid" : "7-N3C3u",
                  "proposedCost" : 10,
                  "proposedCostHint" : [
                     {
                        "isValid" : true,
                        "newCostPrecise" : "-7.7",
                        "newCostRounded" : -10,
                        "unitName" : "Destroyer Snakes"
                     },
                     {
                        "isValid" : true,
                        "newCostPrecise" : "-7.7",
                        "newCostRounded" : -10,
                        "unitName" : "Tri-Scorpions"
                     }
                  ],
                  "proposedVersion" : "2.50",
                  "type" : "ArmyBookUpgradeOption",
                  "uid" : "eDUyrWE"
               },
               "upgrade" : {
                  "affects" : {
                     "type" : "exactly",
                     "value" : 1
                  },
                  "id" : "CyNBUiQ",
                  "label" : "Replace one Dual Reaper Blades",
                  "options" : null,
                  "parentPackageUid" : "K1",
                  "targets" : [
                     "Dual Reaper Blades"
                  ],
                  "type" : "ArmyBookUpgradeSection",
                  "uid" : "7-N3C3u",
                  "variant" : "replace"
               }
            },
            {
               "instanceId" : "ihfIOV7VJ",
               "option" : {
                  "cost" : 5,
                  "costs" : [
                     {
                        "cost" : 5,
                        "unitId" : "gcoBL7U"
                     },
                     {
                        "cost" : 5,
                        "unitId" : "XgJtpyh"
                     }
                  ],
                  "gains" : [
                     {
                        "content" : [
                           {
                              "count" : 3,
                              "dependencies" : [],
                              "label" : "Rending in Melee",
                              "modify" : false,
                              "name" : "Rending in Melee",
                              "type" : "ArmyBookRule"
                           }
                        ],
                        "count" : 3,
                        "dependencies" : [],
                        "label" : "Plasmabot (Rending in Melee)",
                        "name" : "Plasmabot",
                        "type" : "ArmyBookItem"
                     }
                  ],
                  "id" : "CuaPMH-",
                  "label" : "Plasmabot (Rending in Melee)",
                  "parentPackageUid" : "K1",
                  "parentSectionUid" : "TCHi97K",
                  "proposedCostHint" : [
                     {
                        "isValid" : true,
                        "newCostPrecise" : "6.1",
                        "newCostRounded" : 5,
                        "unitName" : "Destroyer Snakes"
                     },
                     {
                        "isValid" : true,
                        "newCostPrecise" : "6.1",
                        "newCostRounded" : 5,
                        "unitName" : "Tri-Scorpions"
                     }
                  ],
                  "proposedVersion" : "2.50",
                  "type" : "ArmyBookUpgradeOption",
                  "uid" : "CuaPMH-"
               },
               "upgrade" : {
                  "affects" : {
                     "type" : "all"
                  },
                  "id" : "I3bLhnp",
                  "label" : "Upgrade all models with",
                  "model" : true,
                  "options" : null,
                  "parentPackageUid" : "K1",
                  "type" : "ArmyBookUpgradeSection",
                  "uid" : "TCHi97K",
                  "variant" : "upgrade"
               }
            }
         ],
         "selectionId" : "6fBQs",
         "size" : 3,
         "specialRules" : [
            {
               "key" : "regeneration",
               "name" : "Regeneration",
               "rating" : ""
            },
            {
               "key" : "robot",
               "name" : "Robot",
               "rating" : ""
            },
            {
               "key" : "strider",
               "name" : "Strider",
               "rating" : ""
            },
            {
               "key" : "tough",
               "name" : "Tough",
               "rating" : "3"
            }
         ],
         "traits" : [],
         "upgrades" : [
            "K1"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "4k5amkxoybdiqotm",
         "combined" : false,
         "cost" : 220,
         "defense" : 3,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 6,
               "count" : 1,
               "id" : "OYdo-1Pu",
               "label" : "Spider Fangs (A6, Poison)",
               "name" : "Spider Fangs",
               "originalCount" : 1,
               "specialRules" : [
                  {
                     "key" : "poison",
                     "modify" : false,
                     "name" : "Poison",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "OYdo-1Pu"
            },
            {
               "attacks" : 6,
               "count" : 1,
               "id" : "4ndrZnDN",
               "label" : "Twin Atom-Beamer (24\", A6, AP(1))",
               "name" : "Twin Atom-Beamer",
               "originalCount" : 1,
               "range" : 24,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "4ndrZnDN"
            }
         ],
         "id" : "obCuik6",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 6,
               "count" : 1,
               "id" : "OYdo-1Pu",
               "label" : "Spider Fangs (A6, Poison)",
               "name" : "Spider Fangs",
               "originalCount" : 1,
               "specialRules" : [
                  {
                     "key" : "poison",
                     "modify" : false,
                     "name" : "Poison",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "OYdo-1Pu"
            },
            {
               "attacks" : 6,
               "count" : 1,
               "id" : "4ndrZnDN",
               "label" : "Twin Atom-Beamer (24\", A6, AP(1))",
               "name" : "Twin Atom-Beamer",
               "originalCount" : 1,
               "range" : 24,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "4ndrZnDN"
            }
         ],
         "name" : "Forge Spider",
         "notes" : null,
         "quality" : 3,
         "selectedUpgrades" : [],
         "selectionId" : "gb1M9",
         "size" : 1,
         "specialRules" : [
            {
               "key" : "regeneration",
               "name" : "Regeneration",
               "rating" : ""
            },
            {
               "key" : "robot",
               "name" : "Robot",
               "rating" : ""
            },
            {
               "key" : "strider",
               "name" : "Strider",
               "rating" : ""
            },
            {
               "key" : "tough",
               "name" : "Tough",
               "rating" : "6"
            }
         ],
         "traits" : [],
         "upgrades" : [
            "N1"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "4k5amkxoybdiqotm",
         "combined" : false,
         "cost" : 175,
         "defense" : 2,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 1,
               "count" : 5,
               "id" : "_WpwGRUk",
               "label" : "CCW (A1)",
               "name" : "CCW",
               "originalCount" : 5,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "4o5CBBZC"
            },
            {
               "attacks" : 2,
               "count" : 5,
               "id" : "dT086NsK",
               "label" : "Flux Carbine (18\", A2, Flux)",
               "name" : "Flux Carbine",
               "originalCount" : 5,
               "range" : 18,
               "specialRules" : [
                  {
                     "key" : "flux",
                     "modify" : false,
                     "name" : "Flux",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "dT086NsK"
            }
         ],
         "id" : "wvgi7_R",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 1,
               "count" : 5,
               "id" : "_WpwGRUk",
               "label" : "CCW (A1)",
               "name" : "CCW",
               "originalCount" : 5,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "4o5CBBZC"
            },
            {
               "attacks" : 2,
               "count" : 5,
               "id" : "dT086NsK",
               "label" : "Flux Carbine (18\", A2, Flux)",
               "name" : "Flux Carbine",
               "originalCount" : 5,
               "range" : 18,
               "specialRules" : [
                  {
                     "key" : "flux",
                     "modify" : false,
                     "name" : "Flux",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "dT086NsK"
            }
         ],
         "name" : "Eternals",
         "notes" : null,
         "quality" : 3,
         "selectedUpgrades" : [],
         "selectionId" : "GY1ll",
         "size" : 5,
         "specialRules" : [
            {
               "key" : "regeneration",
               "name" : "Regeneration",
               "rating" : ""
            },
            {
               "key" : "robot",
               "name" : "Robot",
               "rating" : ""
            },
            {
               "key" : "slow",
               "name" : "Slow",
               "rating" : ""
            }
         ],
         "traits" : [],
         "upgrades" : [
            "F1"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "4k5amkxoybdiqotm",
         "combined" : false,
         "cost" : 75,
         "defense" : 6,
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 3,
               "count" : 3,
               "id" : "DvYpHLlf",
               "label" : "Swarm Attacks (A3, Rending)",
               "name" : "Swarm Attacks",
               "originalCount" : 3,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "bUQRotbI"
            }
         ],
         "id" : "yoeQi4m",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 3,
               "count" : 3,
               "id" : "DvYpHLlf",
               "label" : "Swarm Attacks (A3, Rending)",
               "name" : "Swarm Attacks",
               "originalCount" : 3,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "bUQRotbI"
            }
         ],
         "name" : "Bot Swarms",
         "notes" : null,
         "quality" : 6,
         "selectedUpgrades" : [],
         "selectionId" : "Q7Mth",
         "size" : 3,
         "specialRules" : [
            {
               "key" : "regeneration",
               "name" : "Regeneration",
               "rating" : ""
            },
            {
               "key" : "robot",
               "name" : "Robot",
               "rating" : ""
            },
            {
               "key" : "strider",
               "name" : "Strider",
               "rating" : ""
            },
            {
               "key" : "tough",
               "name" : "Tough",
               "rating" : "3"
            }
         ],
         "traits" : [],
         "upgrades" : [],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "4k5amkxoybdiqotm",
         "combined" : false,
         "cost" : 315,
         "defense" : 3,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 2,
               "count" : 3,
               "id" : "drxT9SeL",
               "label" : "CCW (A2)",
               "name" : "CCW",
               "originalCount" : 3,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "83nhlNHF"
            },
            {
               "attacks" : 3,
               "count" : 3,
               "dependencies" : [
                  {
                     "count" : 1,
                     "upgradeInstanceId" : "i4IiOGCYa",
                     "variant" : "replace"
                  },
                  {
                     "count" : 1,
                     "upgradeInstanceId" : "H3S3cQddT",
                     "variant" : "replace"
                  }
               ],
               "id" : "_FPCvoHf",
               "label" : "Rapid Gauss Rifle (24\", A3, Rending)",
               "name" : "Rapid Gauss Rifle",
               "originalCount" : 3,
               "range" : 24,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "_FPCvoHf"
            }
         ],
         "id" : "yhG8aeu",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 2,
               "count" : 3,
               "id" : "drxT9SeL",
               "label" : "CCW (A2)",
               "name" : "CCW",
               "originalCount" : 3,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "83nhlNHF"
            },
            {
               "attacks" : 3,
               "count" : 1,
               "dependencies" : [
                  {
                     "count" : 1,
                     "upgradeInstanceId" : "i4IiOGCYa",
                     "variant" : "replace"
                  },
                  {
                     "count" : 1,
                     "upgradeInstanceId" : "H3S3cQddT",
                     "variant" : "replace"
                  }
               ],
               "id" : "_FPCvoHf",
               "label" : "Rapid Gauss Rifle (24\", A3, Rending)",
               "name" : "Rapid Gauss Rifle",
               "originalCount" : 3,
               "range" : 24,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "_FPCvoHf"
            },
            {
               "attacks" : 4,
               "count" : 1,
               "dependencies" : [],
               "id" : "HClyzZfI",
               "label" : "Rapid Flux Carbine (18\", A4, Flux)",
               "name" : "Rapid Flux Carbine",
               "range" : 18,
               "specialRules" : [
                  {
                     "id" : 846,
                     "key" : "flux",
                     "name" : "Flux"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "HClyzZfI"
            },
            {
               "attacks" : 3,
               "count" : 1,
               "dependencies" : [],
               "id" : "oGqW8rAE",
               "label" : "Atom-Beamer (24\", A3, AP(1))",
               "name" : "Atom-Beamer",
               "range" : 24,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "label" : "AP(1)",
                     "name" : "AP",
                     "rating" : "1",
                     "type" : "ArmyBookRule"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "oGqW8rAE"
            }
         ],
         "name" : "Hover Bikes",
         "notes" : null,
         "quality" : 3,
         "selectedUpgrades" : [
            {
               "instanceId" : "i4IiOGCYa",
               "option" : {
                  "cost" : -5,
                  "costs" : [
                     {
                        "cost" : -5,
                        "unitId" : "yhG8aeu"
                     }
                  ],
                  "gains" : [
                     {
                        "attacks" : 4,
                        "count" : 1,
                        "dependencies" : [],
                        "id" : "HClyzZfI",
                        "label" : "Rapid Flux Carbine (18\", A4, Flux)",
                        "name" : "Rapid Flux Carbine",
                        "range" : 18,
                        "specialRules" : [
                           {
                              "id" : 846,
                              "key" : "flux",
                              "name" : "Flux"
                           }
                        ],
                        "type" : "ArmyBookWeapon",
                        "weaponId" : "HClyzZfI"
                     }
                  ],
                  "id" : "KUvdZXo",
                  "label" : "Rapid Flux Carbine (18\", A4, Flux)",
                  "parentPackageUid" : "I1",
                  "parentSectionUid" : "lB405hn",
                  "proposedCost" : 10,
                  "proposedCostHint" : [
                     {
                        "isValid" : true,
                        "newCostPrecise" : "-2.9",
                        "newCostRounded" : -5,
                        "unitName" : "Hover Bikes"
                     }
                  ],
                  "proposedVersion" : "2.50",
                  "type" : "ArmyBookUpgradeOption",
                  "uid" : "KUvdZXo"
               },
               "upgrade" : {
                  "affects" : {
                     "type" : "any"
                  },
                  "id" : "JnMMpwY",
                  "label" : "Replace any Rapid Gauss Rifle",
                  "options" : null,
                  "parentPackageUid" : "I1",
                  "targets" : [
                     "Rapid Gauss Rifle"
                  ],
                  "type" : "ArmyBookUpgradeSection",
                  "uid" : "lB405hn",
                  "variant" : "replace"
               }
            },
            {
               "instanceId" : "H3S3cQddT",
               "option" : {
                  "cost" : 5,
                  "costs" : [
                     {
                        "cost" : 5,
                        "unitId" : "yhG8aeu"
                     }
                  ],
                  "gains" : [
                     {
                        "attacks" : 3,
                        "count" : 1,
                        "dependencies" : [],
                        "id" : "oGqW8rAE",
                        "label" : "Atom-Beamer (24\", A3, AP(1))",
                        "name" : "Atom-Beamer",
                        "range" : 24,
                        "specialRules" : [
                           {
                              "key" : "ap",
                              "label" : "AP(1)",
                              "name" : "AP",
                              "rating" : "1",
                              "type" : "ArmyBookRule"
                           }
                        ],
                        "type" : "ArmyBookWeapon",
                        "weaponId" : "oGqW8rAE"
                     }
                  ],
                  "id" : "MA2pB-p",
                  "label" : "Atom-Beamer (24\", A3, AP(1))",
                  "parentPackageUid" : "I1",
                  "parentSectionUid" : "lB405hn",
                  "proposedCost" : 5,
                  "proposedCostHint" : [
                     {
                        "isValid" : true,
                        "newCostPrecise" : "4.8",
                        "newCostRounded" : 5,
                        "unitName" : "Hover Bikes"
                     }
                  ],
                  "proposedVersion" : "2.50",
                  "type" : "ArmyBookUpgradeOption",
                  "uid" : "MA2pB-p"
               },
               "upgrade" : {
                  "affects" : {
                     "type" : "any"
                  },
                  "id" : "JnMMpwY",
                  "label" : "Replace any Rapid Gauss Rifle",
                  "options" : null,
                  "parentPackageUid" : "I1",
                  "targets" : [
                     "Rapid Gauss Rifle"
                  ],
                  "type" : "ArmyBookUpgradeSection",
                  "uid" : "lB405hn",
                  "variant" : "replace"
               }
            }
         ],
         "selectionId" : "GMY9c",
         "size" : 3,
         "specialRules" : [
            {
               "additional" : false,
               "key" : "fast",
               "name" : "Fast",
               "rating" : ""
            },
            {
               "key" : "regeneration",
               "name" : "Regeneration",
               "rating" : ""
            },
            {
               "key" : "robot",
               "name" : "Robot",
               "rating" : ""
            },
            {
               "key" : "strider",
               "name" : "Strider",
               "rating" : ""
            },
            {
               "key" : "tough",
               "name" : "Tough",
               "rating" : "3"
            }
         ],
         "traits" : [],
         "upgrades" : [
            "I1"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "4k5amkxoybdiqotm",
         "combined" : false,
         "cost" : 65,
         "customName" : "Chronomancer",
         "defense" : 3,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 3,
               "count" : 1,
               "id" : "0L3LR8_n",
               "label" : "CCW (A3)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "tO5jEqMO"
            },
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "_g6MjuFl",
               "label" : "Lord Gauss Pistol (12\", A2, Rending)",
               "name" : "Lord Gauss Pistol",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "_g6MjuFl"
            }
         ],
         "id" : "lcEWPMS",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 3,
               "count" : 1,
               "id" : "0L3LR8_n",
               "label" : "CCW (A3)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "tO5jEqMO"
            },
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "_g6MjuFl",
               "label" : "Lord Gauss Pistol (12\", A2, Rending)",
               "name" : "Lord Gauss Pistol",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "_g6MjuFl"
            },
            {
               "content" : [
                  {
                     "count" : 1,
                     "dependencies" : [],
                     "key" : "shadow-protocol",
                     "label" : "Shadow-Protocol",
                     "name" : "Shadow-Protocol",
                     "rating" : "",
                     "type" : "ArmyBookRule"
                  }
               ],
               "count" : 1,
               "dependencies" : [],
               "label" : "Chronomancer (Shadow-Protocol)",
               "name" : "Chronomancer",
               "type" : "ArmyBookItem"
            }
         ],
         "name" : "Robot Lord",
         "notes" : null,
         "quality" : 3,
         "selectedUpgrades" : [
            {
               "instanceId" : "pOMTd9VaT",
               "option" : {
                  "cost" : 25,
                  "costs" : [
                     {
                        "cost" : 25,
                        "unitId" : "lcEWPMS"
                     },
                     {
                        "cost" : 25,
                        "unitId" : "y8xIlHV"
                     },
                     {
                        "cost" : 25,
                        "unitId" : "R2d8LeD"
                     }
                  ],
                  "gains" : [
                     {
                        "content" : [
                           {
                              "count" : 1,
                              "dependencies" : [],
                              "key" : "shadow-protocol",
                              "label" : "Shadow-Protocol",
                              "name" : "Shadow-Protocol",
                              "rating" : "",
                              "type" : "ArmyBookRule"
                           }
                        ],
                        "count" : 1,
                        "dependencies" : [],
                        "label" : "Chronomancer (Shadow-Protocol)",
                        "name" : "Chronomancer",
                        "type" : "ArmyBookItem"
                     }
                  ],
                  "id" : "iUPolg0",
                  "label" : "Chronomancer (Shadow-Protocol)",
                  "parentPackageUid" : "C1",
                  "parentSectionUid" : "IxcS2nP",
                  "proposedCost" : 25,
                  "proposedCostHint" : [
                     {
                        "isValid" : true,
                        "newCostPrecise" : "24.0",
                        "newCostRounded" : 25,
                        "unitName" : "Robot Lord"
                     },
                     {
                        "isValid" : true,
                        "newCostPrecise" : "24.0",
                        "newCostRounded" : 25,
                        "unitName" : "Annihilator Lord"
                     },
                     {
                        "isValid" : true,
                        "newCostPrecise" : "24.0",
                        "newCostRounded" : 25,
                        "unitName" : "Tri-Scorpion Lord"
                     }
                  ],
                  "proposedVersion" : "2.50",
                  "type" : "ArmyBookUpgradeOption",
                  "uid" : "iUPolg0"
               },
               "upgrade" : {
                  "id" : "C_Wx1mp",
                  "isHeroUpgrade" : true,
                  "label" : "Upgrade with one",
                  "options" : null,
                  "parentPackageUid" : "C1",
                  "select" : {
                     "type" : "exactly",
                     "value" : 1
                  },
                  "type" : "ArmyBookUpgradeSection",
                  "uid" : "IxcS2nP",
                  "variant" : "upgrade"
               }
            }
         ],
         "selectionId" : "k8KXN",
         "size" : 1,
         "specialRules" : [
            {
               "key" : "hero",
               "name" : "Hero",
               "rating" : ""
            },
            {
               "key" : "regeneration",
               "name" : "Regeneration",
               "rating" : ""
            },
            {
               "key" : "robot",
               "name" : "Robot",
               "rating" : ""
            },
            {
               "key" : "slow",
               "name" : "Slow",
               "rating" : ""
            },
            {
               "key" : "tough",
               "name" : "Tough",
               "rating" : "3"
            }
         ],
         "traits" : [],
         "upgrades" : [
            "A1",
            "B1",
            "C1"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "4k5amkxoybdiqotm",
         "combined" : false,
         "cost" : 205,
         "defense" : 4,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 2,
               "count" : 6,
               "id" : "IeCvRQZ2",
               "label" : "Metal Fangs (A2, Rending)",
               "name" : "Metal Fangs",
               "originalCount" : 6,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "IeCvRQZ2"
            }
         ],
         "id" : "0x2SCtg",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 2,
               "count" : 6,
               "id" : "IeCvRQZ2",
               "label" : "Metal Fangs (A2, Rending)",
               "name" : "Metal Fangs",
               "originalCount" : 6,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "IeCvRQZ2"
            }
         ],
         "name" : "Robot Snakes",
         "notes" : null,
         "quality" : 3,
         "selectedUpgrades" : [],
         "selectionId" : "Tuc8E",
         "size" : 3,
         "specialRules" : [
            {
               "key" : "regeneration",
               "name" : "Regeneration",
               "rating" : ""
            },
            {
               "key" : "robot",
               "name" : "Robot",
               "rating" : ""
            },
            {
               "key" : "strider",
               "name" : "Strider",
               "rating" : ""
            },
            {
               "key" : "tough",
               "name" : "Tough",
               "rating" : "3"
            }
         ],
         "traits" : [],
         "upgrades" : [
            "J1"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "4k5amkxoybdiqotm",
         "combined" : false,
         "cost" : 205,
         "defense" : 3,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "qMzSue4r",
               "label" : "Annihilation Gun (18\", A2, AP(1), Blast(3))",
               "name" : "Annihilation Gun",
               "originalCount" : 1,
               "range" : 18,
               "specialRules" : [
                  {
                     "key" : "blast",
                     "modify" : false,
                     "name" : "Blast",
                     "rating" : "3"
                  },
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "qMzSue4r"
            },
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "G2NVbL0A",
               "label" : "Harvest Blade (A2, AP(2), Deadly(3))",
               "name" : "Harvest Blade",
               "originalCount" : 1,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "2"
                  },
                  {
                     "key" : "deadly",
                     "modify" : false,
                     "name" : "Deadly",
                     "rating" : "3"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "G2NVbL0A"
            }
         ],
         "id" : "R2d8LeD",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "qMzSue4r",
               "label" : "Annihilation Gun (18\", A2, AP(1), Blast(3))",
               "name" : "Annihilation Gun",
               "originalCount" : 1,
               "range" : 18,
               "specialRules" : [
                  {
                     "key" : "blast",
                     "modify" : false,
                     "name" : "Blast",
                     "rating" : "3"
                  },
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "qMzSue4r"
            },
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "G2NVbL0A",
               "label" : "Harvest Blade (A2, AP(2), Deadly(3))",
               "name" : "Harvest Blade",
               "originalCount" : 1,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "2"
                  },
                  {
                     "key" : "deadly",
                     "modify" : false,
                     "name" : "Deadly",
                     "rating" : "3"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "G2NVbL0A"
            }
         ],
         "name" : "Tri-Scorpion Lord",
         "notes" : null,
         "quality" : 3,
         "selectedUpgrades" : [],
         "selectionId" : "M4smK",
         "size" : 1,
         "specialRules" : [
            {
               "key" : "hero",
               "name" : "Hero",
               "rating" : ""
            },
            {
               "key" : "regeneration",
               "name" : "Regeneration",
               "rating" : ""
            },
            {
               "key" : "robot",
               "name" : "Robot",
               "rating" : ""
            },
            {
               "key" : "relentless",
               "name" : "Relentless",
               "rating" : ""
            },
            {
               "key" : "strider",
               "name" : "Strider",
               "rating" : ""
            },
            {
               "key" : "tough",
               "name" : "Tough",
               "rating" : "6"
            }
         ],
         "traits" : [],
         "upgrades" : [
            "C1"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "4k5amkxoybdiqotm",
         "combined" : false,
         "cost" : 180,
         "defense" : 2,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "0t9U0IHb",
               "label" : "Stomp (A2, AP(1))",
               "name" : "Stomp",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "hyB0WaAL"
            },
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "3OfJ65Xo",
               "label" : "Rapid Particle Beam (12\", A2, AP(1), Blast(3), Reliable)",
               "name" : "Rapid Particle Beam",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  },
                  {
                     "key" : "blast",
                     "modify" : false,
                     "name" : "Blast",
                     "rating" : "3"
                  },
                  {
                     "key" : "reliable",
                     "modify" : false,
                     "name" : "Reliable",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "3OfJ65Xo"
            }
         ],
         "id" : "17TaNC2",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "0t9U0IHb",
               "label" : "Stomp (A2, AP(1))",
               "name" : "Stomp",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "hyB0WaAL"
            },
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "3OfJ65Xo",
               "label" : "Rapid Particle Beam (12\", A2, AP(1), Blast(3), Reliable)",
               "name" : "Rapid Particle Beam",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  },
                  {
                     "key" : "blast",
                     "modify" : false,
                     "name" : "Blast",
                     "rating" : "3"
                  },
                  {
                     "key" : "reliable",
                     "modify" : false,
                     "name" : "Reliable",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "3OfJ65Xo"
            }
         ],
         "name" : "Tripod Walker",
         "notes" : null,
         "quality" : 3,
         "selectedUpgrades" : [],
         "selectionId" : "lIlwJ",
         "size" : 1,
         "specialRules" : [
            {
               "key" : "fear",
               "label" : "Fear(1)",
               "name" : "Fear",
               "rating" : 1
            },
            {
               "key" : "robot",
               "name" : "Robot",
               "rating" : ""
            },
            {
               "key" : "strider",
               "name" : "Strider",
               "rating" : ""
            },
            {
               "key" : "tough",
               "name" : "Tough",
               "rating" : "6"
            }
         ],
         "traits" : [],
         "upgrades" : [
            "A2",
            "D2"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "4k5amkxoybdiqotm",
         "combined" : false,
         "cost" : 65,
         "customName" : "Plasmancer",
         "defense" : 3,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 3,
               "count" : 1,
               "id" : "0L3LR8_n",
               "label" : "CCW (A3)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "tO5jEqMO"
            },
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "_g6MjuFl",
               "label" : "Lord Gauss Pistol (12\", A2, Rending)",
               "name" : "Lord Gauss Pistol",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "_g6MjuFl"
            }
         ],
         "id" : "lcEWPMS",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 3,
               "count" : 1,
               "id" : "0L3LR8_n",
               "label" : "CCW (A3)",
               "name" : "CCW",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [],
               "type" : "ArmyBookWeapon",
               "weaponId" : "tO5jEqMO"
            },
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "_g6MjuFl",
               "label" : "Lord Gauss Pistol (12\", A2, Rending)",
               "name" : "Lord Gauss Pistol",
               "originalCount" : 1,
               "range" : 12,
               "specialRules" : [
                  {
                     "key" : "rending",
                     "modify" : false,
                     "name" : "Rending",
                     "rating" : ""
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "_g6MjuFl"
            },
            {
               "content" : [
                  {
                     "count" : 1,
                     "dependencies" : [],
                     "key" : "gloom-protocol",
                     "label" : "Gloom-Protocol",
                     "name" : "Gloom-Protocol",
                     "rating" : "",
                     "type" : "ArmyBookRule"
                  }
               ],
               "count" : 1,
               "dependencies" : [],
               "label" : "Psychomancer (Gloom-Protocol)",
               "name" : "Psychomancer",
               "type" : "ArmyBookItem"
            }
         ],
         "name" : "Robot Lord",
         "notes" : null,
         "quality" : 3,
         "selectedUpgrades" : [
            {
               "instanceId" : "7QlMfNZYj",
               "option" : {
                  "cost" : 20,
                  "costs" : [
                     {
                        "cost" : 20,
                        "unitId" : "lcEWPMS"
                     },
                     {
                        "cost" : 20,
                        "unitId" : "y8xIlHV"
                     },
                     {
                        "cost" : 20,
                        "unitId" : "R2d8LeD"
                     }
                  ],
                  "gains" : [
                     {
                        "content" : [
                           {
                              "count" : 1,
                              "dependencies" : [],
                              "key" : "gloom-protocol",
                              "label" : "Gloom-Protocol",
                              "name" : "Gloom-Protocol",
                              "rating" : "",
                              "type" : "ArmyBookRule"
                           }
                        ],
                        "count" : 1,
                        "dependencies" : [],
                        "label" : "Psychomancer (Gloom-Protocol)",
                        "name" : "Psychomancer",
                        "type" : "ArmyBookItem"
                     }
                  ],
                  "id" : "VHtYlsj",
                  "label" : "Psychomancer (Gloom-Protocol)",
                  "parentPackageUid" : "C1",
                  "parentSectionUid" : "IxcS2nP",
                  "proposedCost" : 25,
                  "proposedCostHint" : [
                     {
                        "isValid" : true,
                        "newCostPrecise" : "21.1",
                        "newCostRounded" : 20,
                        "unitName" : "Robot Lord"
                     },
                     {
                        "isValid" : true,
                        "newCostPrecise" : "21.1",
                        "newCostRounded" : 20,
                        "unitName" : "Annihilator Lord"
                     },
                     {
                        "isValid" : true,
                        "newCostPrecise" : "21.1",
                        "newCostRounded" : 20,
                        "unitName" : "Tri-Scorpion Lord"
                     }
                  ],
                  "proposedVersion" : "2.50",
                  "type" : "ArmyBookUpgradeOption",
                  "uid" : "VHtYlsj"
               },
               "upgrade" : {
                  "id" : "C_Wx1mp",
                  "isHeroUpgrade" : true,
                  "label" : "Upgrade with one",
                  "options" : null,
                  "parentPackageUid" : "C1",
                  "select" : {
                     "type" : "exactly",
                     "value" : 1
                  },
                  "type" : "ArmyBookUpgradeSection",
                  "uid" : "IxcS2nP",
                  "variant" : "upgrade"
               }
            }
         ],
         "selectionId" : "BbWjg",
         "size" : 1,
         "specialRules" : [
            {
               "key" : "hero",
               "name" : "Hero",
               "rating" : ""
            },
            {
               "key" : "regeneration",
               "name" : "Regeneration",
               "rating" : ""
            },
            {
               "key" : "robot",
               "name" : "Robot",
               "rating" : ""
            },
            {
               "key" : "slow",
               "name" : "Slow",
               "rating" : ""
            },
            {
               "key" : "tough",
               "name" : "Tough",
               "rating" : "3"
            }
         ],
         "traits" : [],
         "upgrades" : [
            "A1",
            "B1",
            "C1"
         ],
         "valid" : true,
         "xp" : 0
      },
      {
         "armyId" : "4k5amkxoybdiqotm",
         "combined" : false,
         "cost" : 450,
         "defense" : 2,
         "disabledSections" : [],
         "disabledUpgradeSections" : [],
         "equipment" : [
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "mF_XO4Hl",
               "label" : "Crushing Legs (A2, AP(2))",
               "name" : "Crushing Legs",
               "originalCount" : 1,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "2"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "mF_XO4Hl"
            },
            {
               "attacks" : 4,
               "count" : 1,
               "id" : "04wtW0y-",
               "label" : "Stomp (A4, AP(1))",
               "name" : "Stomp",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "l9ncuAr5"
            },
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "NiErApPd",
               "label" : "Twin Fusion Ray (18\", A2, AP(4), Deadly(6))",
               "name" : "Twin Fusion Ray",
               "originalCount" : 1,
               "range" : 18,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "4"
                  },
                  {
                     "key" : "deadly",
                     "modify" : false,
                     "name" : "Deadly",
                     "rating" : "6"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "NiErApPd"
            }
         ],
         "id" : "n9nkDBw",
         "joinToUnit" : null,
         "loadout" : [
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "mF_XO4Hl",
               "label" : "Crushing Legs (A2, AP(2))",
               "name" : "Crushing Legs",
               "originalCount" : 1,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "2"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "mF_XO4Hl"
            },
            {
               "attacks" : 4,
               "count" : 1,
               "id" : "04wtW0y-",
               "label" : "Stomp (A4, AP(1))",
               "name" : "Stomp",
               "originalCount" : 1,
               "range" : 0,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "1"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "l9ncuAr5"
            },
            {
               "attacks" : 2,
               "count" : 1,
               "id" : "NiErApPd",
               "label" : "Twin Fusion Ray (18\", A2, AP(4), Deadly(6))",
               "name" : "Twin Fusion Ray",
               "originalCount" : 1,
               "range" : 18,
               "specialRules" : [
                  {
                     "key" : "ap",
                     "modify" : false,
                     "name" : "AP",
                     "rating" : "4"
                  },
                  {
                     "key" : "deadly",
                     "modify" : false,
                     "name" : "Deadly",
                     "rating" : "6"
                  }
               ],
               "type" : "ArmyBookWeapon",
               "weaponId" : "NiErApPd"
            }
         ],
         "name" : "Spider Walker",
         "notes" : null,
         "quality" : 3,
         "selectedUpgrades" : [],
         "selectionId" : "nXvZ_",
         "size" : 1,
         "specialRules" : [
            {
               "key" : "fear",
               "label" : "Fear(2)",
               "name" : "Fear",
               "rating" : 2
            },
            {
               "key" : "robot",
               "name" : "Robot",
               "rating" : ""
            },
            {
               "key" : "strider",
               "name" : "Strider",
               "rating" : ""
            },
            {
               "key" : "tough",
               "name" : "Tough",
               "rating" : "12"
            }
         ],
         "traits" : [],
         "upgrades" : [
            "E2"
         ],
         "valid" : true,
         "xp" : 0
      }
   ]
}
"#,
];
