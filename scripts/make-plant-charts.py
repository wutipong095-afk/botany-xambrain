# -*- coding: utf-8 -*-
"""สร้างผังจำแนกพืช (SVG) 2 แบบ: รูปวิธาน และ อนุกรมวิธาน"""
import io

FONT = "TH Sarabun New, Leelawadee UI, Tahoma, sans-serif"
FS = 30           # font size
LH = 38           # line height
PAD_X = 26
BOX_1 = 62        # height for 1 line
BOX_2 = 100       # height for 2 lines

COL_DECISION = ("#FFF8E6", "#B8860B")   # กล่องลักษณะที่ใช้แยก
COL_OPTION = ("#FFFFFF", "#4A6FA5")     # กล่องตัวเลือก A/B
COL_RESULT = ("#EAF5EA", "#2F7D32")     # กล่องผลลัพธ์ = ชื่อพืช
COL_ROOT = ("#E8F0FA", "#1F4E79")
COL_RANK = ("#F4F1E8", "#7A6A45")


def w_of(lines):
    """ประมาณความกว้างกล่องจากความยาวข้อความ"""
    longest = max(len(l) for l in lines)
    return int(longest * 14.6) + PAD_X * 2


class Canvas:
    def __init__(self, w, h):
        self.w, self.h = w, h
        self.parts = []
        self.boxes = {}

    def box(self, key, cx, y, lines, colors, italic_last=False, w=None):
        bw = w or w_of(lines)
        h = BOX_1 if len(lines) == 1 else BOX_2
        x = cx - bw / 2
        fill, stroke = colors
        self.parts.append(
            f'<rect x="{x:.0f}" y="{y}" width="{bw}" height="{h}" rx="8" '
            f'fill="{fill}" stroke="{stroke}" stroke-width="2.5"/>'
        )
        total = len(lines) * LH
        start = y + (h - total) / 2 + LH * 0.72
        for i, line in enumerate(lines):
            style = ''
            if italic_last and i == len(lines) - 1:
                style = ' font-style="italic" fill="#333"'
            self.parts.append(
                f'<text x="{cx:.0f}" y="{start + i * LH:.0f}" text-anchor="middle" '
                f'font-family="{FONT}" font-size="{FS}"{style}>{line}</text>'
            )
        self.boxes[key] = (cx, y, bw, h)
        return key

    def link(self, a, b, drop=None):
        """เส้นเชื่อมแบบหักฉาก จากล่างกล่อง a ไปบนกล่อง b"""
        ax, ay, aw, ah = self.boxes[a]
        bx, by, bw, bh = self.boxes[b]
        y1 = ay + ah
        mid = drop or (y1 + (by - y1) / 2)
        d = f"M {ax:.0f} {y1} L {ax:.0f} {mid:.0f} L {bx:.0f} {mid:.0f} L {bx:.0f} {by - 12}"
        self.parts.append(
            f'<path d="{d}" fill="none" stroke="#444" stroke-width="2.5" marker-end="url(#arrow)"/>'
        )

    def label(self, x, y, text, size=28, anchor="start", weight="normal", color="#111"):
        self.parts.append(
            f'<text x="{x}" y="{y}" text-anchor="{anchor}" font-family="{FONT}" '
            f'font-size="{size}" font-weight="{weight}" fill="{color}">{text}</text>'
        )

    def save(self, path):
        head = (
            f'<svg xmlns="http://www.w3.org/2000/svg" width="{self.w}" height="{self.h}" '
            f'viewBox="0 0 {self.w} {self.h}">'
            '<defs><marker id="arrow" viewBox="0 0 10 10" refX="9" refY="5" '
            'markerWidth="7" markerHeight="7" orient="auto-start-reverse">'
            '<path d="M 0 0 L 10 5 L 0 10 z" fill="#444"/></marker></defs>'
            f'<rect width="{self.w}" height="{self.h}" fill="#FFFFFF"/>'
        )
        io.open(path, "w", encoding="utf-8").write(head + "".join(self.parts) + "</svg>")


# ---------------- ผังที่ 1 : ตามรูปวิธาน ----------------
def diagram_key(path):
    c = Canvas(3160, 1460)
    r = [40, 190, 350, 500, 670, 830, 990, 1150]

    c.box("root", 1300, r[0], ["แผนผังการจำแนกพืชศึกษา 6 ชนิด"], COL_ROOT)

    c.box("1A", 520, r[1], ["1A  ไม้ต้น · ใบเดี่ยวเรียงตรงข้าม", "(ตะแบกนา, อินทนิลบก)"], COL_OPTION)
    c.box("1B", 1900, r[1], ["1B  ไม้พุ่ม / ล้มลุก / ไม้เถา · ใบเรียงสลับ", "(หม่อน, กัญชา, อัญชัน, เถาวัลย์เปรียง)"], COL_OPTION)
    c.link("root", "1A", drop=r[0] + BOX_1 + 45)
    c.link("root", "1B", drop=r[0] + BOX_1 + 45)

    # ---- กิ่งซ้าย ----
    c.box("d2", 520, r[2], ["2. ลักษณะเปลือกต้น + ขนาดผล"], COL_DECISION)
    c.link("1A", "d2")
    c.box("2A", 260, r[3], ["2A  เปลือกลอกเป็นแผ่น", "ผลยาว 1.5–2 ซม."], COL_OPTION)
    c.box("2B", 790, r[3], ["2B  เปลือกแตกสะเก็ด", "ผลยาว 3–4 ซม."], COL_OPTION)
    c.link("d2", "2A")
    c.link("d2", "2B")
    c.box("sp1", 260, r[4], ["ตะแบกนา", "Lagerstroemia floribunda"], COL_RESULT, italic_last=True)
    c.box("sp2", 790, r[4], ["อินทนิลบก", "Lagerstroemia macrocarpa"], COL_RESULT, italic_last=True)
    c.link("2A", "sp1")
    c.link("2B", "sp2")

    # ---- กิ่งขวา ----
    c.box("d3", 1900, r[2], ["3. ชนิดใบ + ยางสีขาว"], COL_DECISION)
    c.link("1B", "d3")
    c.box("3A", 1560, r[3], ["3A  ใบเดี่ยว มียางขาว", "เส้นใบออกจากโคน 3 เส้น"], COL_OPTION)
    c.box("3B", 2270, r[3], ["3B  ใบประกอบ ไม่มียางขาว"], COL_OPTION)
    c.link("d3", "3A")
    c.link("d3", "3B")
    c.box("sp3", 1560, r[4], ["หม่อน", "Morus alba"], COL_RESULT, italic_last=True)
    c.link("3A", "sp3")

    c.box("d4", 2290, r[4], ["4. แบบของใบประกอบ"], COL_DECISION)
    c.link("3B", "d4")
    c.box("4A", 2040, r[5], ["4A  แบบนิ้วมือ", "ขอบจักฟันเลื่อยลึก"], COL_OPTION)
    c.box("4B", 2560, r[5], ["4B  แบบขนนกปลายคี่", "ขอบเรียบ · ไม้เถา"], COL_OPTION)
    c.link("d4", "4A")
    c.link("d4", "4B")
    c.box("sp4", 2040, r[6], ["กัญชา", "Cannabis sativa"], COL_RESULT, italic_last=True)
    c.link("4A", "sp4")

    c.box("d5", 2590, r[6], ["5. ชนิดเถา + ช่อดอก"], COL_DECISION)
    c.link("4B", "d5")
    c.box("5A", 2350, r[7], ["5A  เถาล้มลุก · ดอกเดี่ยวใหญ่"], COL_OPTION)
    c.box("5B", 2840, r[7], ["5B  เถาเนื้อแข็ง · ช่อกระจะยาว"], COL_OPTION)
    c.link("d5", "5A")
    c.link("d5", "5B")

    # ผลลัพธ์สองชนิดสุดท้ายวางเยื้องด้านซ้ายของแถวเดียวกัน
    c.box("sp5", 2350, r[7] + 140, ["อัญชัน", "Clitoria ternatea"], COL_RESULT, italic_last=True)
    c.box("sp6", 2840, r[7] + 140, ["เถาวัลย์เปรียง", "Derris scandens"], COL_RESULT, italic_last=True)
    c.link("5A", "sp5")
    c.link("5B", "sp6")

    c.label(40, 1420, "กล่องสีเหลือง = ลักษณะที่ใช้แยก · กล่องขาว = ตัวเลือก A/B · กล่องเขียว = ชนิดพืชที่ระบุได้", size=26, color="#555")
    c.save(path)


# ---------------- ผังที่ 2 : ตามอนุกรมวิธาน ----------------
def diagram_taxonomy(path):
    c = Canvas(2760, 1180)
    r = [40, 170, 300, 450, 590, 720, 860]

    c.box("k", 1380, r[0], ["อาณาจักร Plantae  (พืช)"], COL_ROOT)
    c.box("d", 1380, r[1], ["ดิวิชัน Magnoliophyta  (พืชดอก)"], COL_RANK)
    c.box("cl", 1380, r[2], ["ชั้น Magnoliopsida  (พืชใบเลี้ยงคู่)"], COL_RANK)
    c.link("k", "d")
    c.link("d", "cl")

    c.box("o1", 480, r[3], ["อันดับ Myrtales", "(malvids)"], COL_DECISION)
    c.box("o2", 1380, r[3], ["อันดับ Rosales", "(fabids)"], COL_DECISION)
    c.box("o3", 2260, r[3], ["อันดับ Fabales", "(fabids)"], COL_DECISION)
    for o in ("o1", "o2", "o3"):
        c.link("cl", o, drop=r[2] + BOX_1 + 45)

    c.box("f1", 480, r[4], ["วงศ์ Lythraceae"], COL_OPTION)
    c.box("f2", 1140, r[4], ["วงศ์ Moraceae"], COL_OPTION)
    c.box("f3", 1650, r[4], ["วงศ์ Cannabaceae"], COL_OPTION)
    c.box("f4", 2260, r[4], ["วงศ์ Fabaceae"], COL_OPTION)
    c.link("o1", "f1")
    c.link("o2", "f2")
    c.link("o2", "f3")
    c.link("o3", "f4")

    c.box("g1", 480, r[5], ["สกุล Lagerstroemia"], COL_OPTION)
    c.box("g2", 1140, r[5], ["สกุล Morus"], COL_OPTION)
    c.box("g3", 1650, r[5], ["สกุล Cannabis"], COL_OPTION)
    c.box("g4", 2080, r[5], ["สกุล Clitoria"], COL_OPTION)
    c.box("g5", 2500, r[5], ["สกุล Derris"], COL_OPTION)
    c.link("f1", "g1")
    c.link("f2", "g2")
    c.link("f3", "g3")
    c.link("f4", "g4")
    c.link("f4", "g5")

    c.box("s1", 250, r[6], ["ตะแบกนา", "L. floribunda"], COL_RESULT, italic_last=True)
    c.box("s2", 700, r[6], ["อินทนิลบก", "L. macrocarpa"], COL_RESULT, italic_last=True)
    c.box("s3", 1140, r[6], ["หม่อน", "M. alba"], COL_RESULT, italic_last=True)
    c.box("s4", 1650, r[6], ["กัญชา", "C. sativa"], COL_RESULT, italic_last=True)
    c.box("s5", 2080, r[6], ["อัญชัน", "C. ternatea"], COL_RESULT, italic_last=True)
    c.box("s6", 2510, r[6], ["เถาวัลย์เปรียง", "D. scandens"], COL_RESULT, italic_last=True)
    c.link("g1", "s1")
    c.link("g1", "s2")
    c.link("g2", "s3")
    c.link("g3", "s4")
    c.link("g4", "s5")
    c.link("g5", "s6")

    c.label(40, 1030, "หมายเหตุ: malvids และ fabids เป็นการจัดกลุ่มใหญ่ในกลุ่ม Rosids ตามระบบ APG (ตำราเก่าใช้คำว่า ชั้นย่อย Rosidae)", size=26, color="#555")
    c.label(40, 1075, "หม่อนกับกัญชาอยู่อันดับเดียวกัน (Rosales) จึงมีดอกลดรูป ไม่มีกลีบดอก และผสมเกสรโดยลมเหมือนกัน", size=26, color="#555")
    c.label(40, 1120, "อัญชันกับเถาวัลย์เปรียงอยู่วงศ์เดียวกัน (Fabaceae) เพราะดอกรูปดอกถั่วและผลเป็นฝัก ไม่ใช่เพราะเป็นไม้เถา", size=26, color="#555")
    c.save(path)


diagram_key("chart-key.svg")
diagram_taxonomy("chart-taxonomy.svg")
print("svg written")
