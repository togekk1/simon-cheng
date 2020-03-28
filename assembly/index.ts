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

export class main {
  html_global: string;
  prod: boolean = false;

  constructor(html: string) {
    const iter_arr: string[] = ['style', 'class', 'disabled'],
      scopes: Map<string, boolean> = new Map();

    scopes.set('prod', this.prod);

    let html_iter_arr: string[],
      to_process: string,
      split_by_semicon: string[],
      split_by_question_mark: string[],
      value: string;

    for (let i = 0; i < iter_arr.length; i++) {
      html_iter_arr = html.split('[' + iter_arr[i] + ']=\"');
      for (let j = 0; j < html_iter_arr.length; j++)
        if (j > 0) {
          to_process = html_iter_arr[j].substring(0, html_iter_arr[j].indexOf('\"'));
          if (iter_arr[i] === 'style') {
            split_by_semicon = to_process.substring(2, to_process.indexOf('}')).replace('\'', '').split(':');
            split_by_question_mark = split_by_semicon[1].split('?');
            value = scopes.get(/* scope */split_by_question_mark[0].trim()) ?
              /* true case */split_by_question_mark[1].trim().substring(1, split_by_question_mark[1].trim().length - 1)
              :
              /* false case */split_by_semicon[2].trim().substring(1, split_by_semicon[2].trim().length - 1);
            html = html.replace(
              '[' + iter_arr[i] + ']=\"' + to_process + '"',
              iter_arr[i] + '=\"' + split_by_semicon[0] + ':' + value + '\"'
            );
          };
          // variables.set(iter_arr[i] + (j - 1).toString(), html_iter_arr[j].substring(0, html_iter_arr[j].indexOf('\"')))
        }
    };
    render(html);
  }
}

// export function click_event(): void {
//   let html = html_global.replace(`{{msg}}`, event_global);
//   render(html);
// }