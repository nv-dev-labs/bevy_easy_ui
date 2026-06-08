(
    - EasyScroller
    ou
    - EasyVerticalScroller
    - EasyHorizontalScroller
)
ET
- Mettre Styles en tuple et non en struct

---

# TODO — bevy_easy_ui

Légende : `[feat]` feature · `[fix]` bug · `[refactor]` dette technique · `[chore]` méta.

## 1 — Wrappers manquants des widgets Bevy non natifs
- [ ] [feat] `EasyTextInput` — pas natif, à construire (Node + Text + state + keyboard).

## 5 — Tests & doc
- [ ] [feat] Tests unitaires : `Container::spawn` consomme les observers ; `with_overflow(Overflow::scroll())` se propage dans le `Node`.
- [ ] [chore] Doc-tests sur les setters `with_*` publics.
- [ ] [feat] Section "Limitations" dans le README (pas de gestion d'état, pas de hot-reload, etc.).

## 6 — Hors-scope
- [ ] Animations / tweening (`bevy_easy_ui_anim`).
- [ ] Theming / stylesheets (`.ron` → `EasyBoxStyle` au startup).
- [ ] Hot-reload des styles.
- [ ] Composant `EasyInteractionState` (hovered / pressed) partagé entre widgets — la lib se contente d'attacher les observers, l'état est porté par l'utilisateur.

## 7 — Release checklist 0.1.0
- [ ] Au moins un smoke test qui spawn la lib et vérifie qu'aucune panic au démarrage.
- [ ] `cargo doc` généré sans warning sur doc.rs.
- [ ] README + CHANGELOG synchronisés.
