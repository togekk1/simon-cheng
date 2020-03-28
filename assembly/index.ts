// The entry file of your WebAssembly module.

@external("env", "logi")
declare function logi(val: i32): void;
@external("env", "logf")
declare function logf(val: f64): void;
@external("env", "log")
declare function log(val: string): void;
@external("env", "render")
declare function render(html: string): void;
@external("env", "add_event_listener")
declare function add_event_listener(id: string): void;

let html_global: string,
  event_global: string;

export function main(html: string, event: string): void {
  html_global = html;
  const variables: Map<string, string> = new Map();
  variables.set('msg', 'Hello, World!');
  event = event.replace('msg = \'', '');
  event = event.substr(0, event.length - 1);
  html = html.replace(`{{msg}}`, variables.get('msg'));
  render(html);
  add_event_listener('test');
  event_global = event;
}

export function click_event(): void {
  let html = html_global.replace(`{{msg}}`, event_global);
  render(html);
}