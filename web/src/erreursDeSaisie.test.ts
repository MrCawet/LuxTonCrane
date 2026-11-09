import { expect, test } from 'vitest'
import { erreursDeLAgendaDuMatin } from './erreursDeSaisie'

test("un agenda du matin complet n'a pas d'erreur", () => {
  expect(
    erreursDeLAgendaDuMatin({ endormissement: 'calme', continuite: 'sans_reveil', coucher: '23:07', lever: '07:52' }),
  ).toEqual([])
})

test("chaque oubli de l'agenda du matin est dit", () => {
  expect(erreursDeLAgendaDuMatin({ endormissement: '', continuite: '', coucher: '', lever: '' })).toEqual([
    "Choisis un niveau pour l'endormissement.",
    'Choisis un niveau pour la continuité de la nuit.',
    "Indique l'heure du coucher.",
    "Indique l'heure du lever.",
  ])
})

test('un coucher et un lever à la même heure sont refusés', () => {
  expect(
    erreursDeLAgendaDuMatin({ endormissement: 'calme', continuite: 'sans_reveil', coucher: '07:00', lever: '07:00' }),
  ).toEqual(['Le coucher et le lever ne peuvent pas être à la même heure.'])
})
