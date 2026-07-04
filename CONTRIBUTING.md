# Contributing — Git & Workflow

มาตรฐาน Git สำหรับ **botany-xambrain** (การทำสมองที่สอง)

## Branch

| กฎ | รายละเอียด |
|----|-------------|
| **Default branch** | `master` |
| **ห้าม commit ตรง master** | สร้าง branch ก่อนเสมอ (ดู `.cursor/rules/no-work-on-main.mdc`) |
| **Merge ผ่าน PR** | ไม่ push ตรง master · รอ CI ผ่านก่อน merge |

### ชื่อ branch

```
<type>/<short-description>
```

| type | ใช้เมื่อ |
|------|----------|
| `content/` | เนื้อหา wiki, PDF, รูป, qbank |
| `feature/` | ฟีเจอร์ใหม่ (app, recommender) |
| `fix/` | แก้บั๊ก |
| `chore/` | CI, git config, docs, refactor เล็ก |

ตัวอย่าง: `content/flower-leaf-sources` · `fix/recommender-scoring` · `chore/git-standards`

## Commit

- **ภาษาอังกฤษ** · 1–2 ประโยข · เน้น *why*
- รูปแบบ: `Add …` · `Fix …` · `Update …` · `Remove …`

```
Add knowledge extraction pipeline and fix review typos

Introduce check-text script, enforce LF line endings, fix morphology typos.
```

## Pull Request

1. `git checkout -b <type>/<name>` จาก `master` ล่าสุด
2. ทำงาน + commit
3. `git push -u origin HEAD`
4. เปิด PR → `master` (template มี checklist)
5. รอ **CI** (`check-text`) เป็นสีเขียว
6. Review แล้ว merge (squash หรือ merge commit ตาม repo)

## CI (GitHub Actions)

| Job | สคริปต์ | ตรวจอะไร |
|-----|---------|----------|
| `check-text` | `scripts/check-text.py` | typo, อักษร Latin ปนไทย, path รูป wikilink |

รัน local ก่อน push:

```bash
python scripts/check-text.py
```

## Line endings

- Repo ใช้ **LF** ทั้งหมด (`.gitattributes`: `* text=auto eol=lf`)
- สคริปต์ Python เขียนไฟล์ด้วย `newline="\n"` เสมอ

## เนื้อหา wiki

Pipeline ย่อยความรู้: [[knowledge-extraction-pipeline]] · คำสั่ง Cursor: `CLAUDE.md`

## Branch protection (maintainer)

ตั้งบน GitHub สำหรับ `master`:

- Require pull request before merging
- Require status check: **check-text**
- ไม่ allow force push

```bash
gh api repos/{owner}/{repo}/branches/master/protection -X PUT \
  -f required_status_checks[strict]=true \
  -f required_status_checks[contexts][]=check-text \
  -f enforce_admins=false \
  -f required_pull_request_reviews[required_approving_review_count]=0 \
  -f restrictions=null
```

(ปรับ `required_approving_review_count` เป็น 1 ถ้าต้องการ review บังคับ)
