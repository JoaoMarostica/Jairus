export function normalizeText(text: string | null): string {
  return text
    ? text.trim().toLowerCase().normalize('NFD').replace(/[\u0300-\u036f]/g, '')
    : '';
}

export function getStatusType(status: number) {
  switch (status) {
    case 1:
      return 'primary'
    case 0:
      return 'warning'
    default:
      return 'error'
  }
}

export function getStatusLabel(status: number) {
  switch (status) {
    case 1:
      return 'Ativo'
    case 0:
      return 'Encerrado'
    default:
      return 'erro'
  }
}


export function parseExpireDate(dateStr: string): Date | null {
  if (!dateStr) return null;

  const parts = dateStr.toLowerCase().split('/');
  if (parts.length !== 2) return null;

  const month = monthMap[parts[0]];
  const year = Number(parts[1]);

  if (month === undefined || isNaN(year)) return null;

  return new Date(year, month, 1);
}

const monthMap: Record<string, number> = {
  jan: 0,
  fev: 1,
  mar: 2,
  abr: 3,
  mai: 4,
  jun: 5,
  jul: 6,
  ago: 7,
  set: 8,
  out: 9,
  nov: 10,
  dez: 11,
}

export function parseBrazilianNumber(value: string | number): number {
  if (typeof value === 'number') return value
  if (!value) return 0
  return parseFloat(value.replace(/\./g, '').replace(',', '.'))
}
