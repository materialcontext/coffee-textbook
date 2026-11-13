# Upstream house style

Every chapter and appendix is written to this guide. The build enforces the parts a program can check; the rest is checked by editorial review.

## 1. Reader

A second-year undergraduate in a science or engineering degree, or an equally prepared professional. Assume:

- first-year general chemistry: moles, concentration, balanced equations, the idea of equilibrium and pH;
- first-year calculus: derivatives, integrals, exponentials and logarithms, solving dy/dt = &minus;ky;
- no organic chemistry beyond recognising a structural formula, no physical chemistry, no transport phenomena, no statistics beyond a mean, no sensory science, no coffee trade knowledge.

Anything outside that list is taught in the text or in Appendices A to E at its first use, or placed in a "Going further" box that the main line of the chapter does not depend on.

## 2. Terms

- Define every technical term at first use, with `{{term}}` markup, or `{{glossary form|form in this sentence}}` when the sentence needs a plural or other form (rendered as a defined term and collected into the chapter's key-terms list). The definition sits in the same sentence or the next one.
- One term per idea. Do not vary terms for elegance: "extraction yield" stays "extraction yield".
- Abbreviations: spell out at first use in each chapter, then abbreviate. Keep to those that readers will meet elsewhere (TDS, EY, SCA, HPLC).
- Every glossary term is defined in the text somewhere; the build checks this.

## 3. Notation (book-wide; also in the Notation page)

| Symbol | Meaning | Unit |
|---|---|---|
| m<sub>c</sub> | dose of dry coffee | g |
| m<sub>w</sub> | brew water (Chapter 7: water in beans) | g |
| m<sub>b</sub> | beverage | g |
| m<sub>s</sub> | dissolved solids | g |
| S | strength, TDS as a mass fraction | dimensionless (quote as %) |
| E | extraction yield | dimensionless (quote as %) |
| r | brew ratio m<sub>w</sub>/m<sub>c</sub> | g/g |
| K, E<sub>max</sub> | extraction equilibrium constant; maximum extractable fraction | dimensionless |
| D | diffusivity | m<sup>2</sup>/s |
| a, d | particle radius, diameter | m (&micro;m in prose) |
| &tau; | time constant | s or min |
| &kappa; | permeability | m<sup>2</sup> |
| &epsilon; | porosity | dimensionless |
| &mu; | dynamic viscosity | Pa s |
| &rho; | density | kg/m<sup>3</sup> |
| T | temperature | K in formulas, &deg;C in prose |
| k | rate constant | 1/s or 1/min |
| E<sub>a</sub> | activation energy | J/mol |
| R | gas constant, 8.314 J/(mol K) | |
| &lambda; | thermal conductivity | W/(m K) |
| h | heat-transfer coefficient | W/(m<sup>2</sup> K) |
| c<sub>p</sub> | specific heat capacity | J/(kg K) |
| &gamma; | heating rate dT/dt | &deg;C/min |
| &beta; | Weibull shape parameter | dimensionless |
| w, X | moisture, wet and dry basis | dimensionless |

Never reuse a symbol for a different quantity within a chapter. A sentence never starts with a symbol.

## 4. Equations

- Important relations are display equations (`~~~eq` blocks), numbered by chapter (2A.3), with every symbol and its unit listed under the equation the first time it appears.
- Refer to them by number and by name: "the immersion mass balance, Equation 2A.3". In the source, write `{eq:id}` alone; it expands to "Equation 2A.3" (and `{fig:id}` to "Figure 2A.1"). The build rejects "Equation {eq:id}".
- Derivations show every algebraic step a second-year student would need; steps that require differential equations beyond first order go in a "Going further" box with the result restated in the main text.
- Explain what an equation says in words before or after it, and what it predicts (sign, scaling, limiting cases).

## 5. Chapter anatomy

1. Objectives (generated from front matter).
2. Opener: one to three paragraphs that pose a concrete coffee problem the chapter will solve. No history lessons, no hype.
3. Numbered sections. Each opens with a sentence linking it to the previous one and stating its question. One main idea per section; long sections are split.
4. A short "Check your understanding" quiz after most sections (retrieval practice).
5. Worked examples with four labelled parts: Problem, Plan, Solution, Check. Later examples in a chapter leave one step to the reader ("Your turn"), fading support.
6. "Common misconception" boxes where a belief is widespread in coffee practice: state the belief, say why it is plausible, then give the evidence against it.
7. Evidence boxes for single-study, preprint or contested claims.
8. Figures referred to by number in the text before they appear; captions say what to notice.
9. "Looking ahead": one paragraph that names the question the next chapter answers and why it follows. No recap.
10. Key equations, key terms (generated), labs, problems (tagged with the section they practise), references.

## 6. Sentences and paragraphs

- Old information at the start of a sentence, new information at the end (Gopen and Swan). Subject close to verb. Actions in verbs.
- Average 18 to 22 words per sentence; none above 40. One new idea per sentence where the idea is hard.
- Paragraphs open with their point. No paragraph without a function.
- "We" means author and reader working together; "you" addresses the reader doing a lab or a calculation.
- Numbers: spell out one to nine in prose except with units; always a space between number and unit; percentages as "21.5%".

## 7. Banned

The prose linter rejects em dashes, bold for emphasis, and stock phrases associated with machine-written or padded text (see `src/lint.rs`). Also avoid: "it is important to note", "in other words" as a crutch, rhetorical questions in place of statements, "simply", "obviously", "clearly", sentences beginning with "This" with no noun, praise of cited work ("elegant", "landmark"), and summaries that repeat the section.
