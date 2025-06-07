export function parsePtBrNumber(value: string | null): number {
  if (!value) return 0
  return Number(value.replace(/\./g, '').replace(',', '.'))
}

export function parseNumber(input: string): number | null {
  const cleaned = input.trim()
    .replace(/\./g, '')
    .replace(',', '.')

  if (cleaned === '') return null

  const num = Number(cleaned)
  return isNaN(num) ? Number.NaN : num
}

export function formatNumber(value: number | null): string {
  if (value === null)
    return ''
  return value.toLocaleString('pt-BR', {
    minimumFractionDigits: 1,
    maximumFractionDigits: 2
  })
}

export function parseExpireDate(expireDate: number, year: number): string {
  const monthNames = ['jan', 'fev', 'mar', 'abr', 'mai', 'jun', 'jul', 'ago', 'set', 'out', 'nov', 'dez'];
  const month = monthNames[expireDate] || '--';
  return `${month}/${year + 1}`;
}

