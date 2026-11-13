// Loads every built page in a DOM, runs book.js, and checks that each
// calculator produces output and each quiz marks correct answers as correct.
// Run: node pages.test.js <dist-dir>
var fs = require('fs');
var path = require('path');
var assert = require('assert');
var JSDOM = require('jsdom').JSDOM;

var dist = path.resolve(process.argv[2] || path.join(__dirname, '..', '..', 'dist'));
var js = fs.readFileSync(path.join(dist, 'book.js'), 'utf8');
var pages = fs.readdirSync(dist).filter(function (f) { return /\.html$/.test(f); });
var widgets = 0, quizzes = 0, questions = 0;

pages.forEach(function (name) {
  var html = fs.readFileSync(path.join(dist, name), 'utf8')
    .replace('<script type="text/javascript" src="book.js"></script>', '');
  var dom = new JSDOM(html, { runScripts: 'outside-only' });
  var w = dom.window, d = w.document;
  w.eval(js);
  w.UB.init();

  // Every internal link target exists.
  Array.prototype.forEach.call(d.querySelectorAll('a[href]'), function (a) {
    var href = a.getAttribute('href');
    if (/^#/.test(href)) {
      assert(d.getElementById(href.slice(1)), name + ': missing anchor ' + href);
    } else if (!/^[a-z]+:/.test(href)) {
      assert(fs.existsSync(path.join(dist, href.split('#')[0])), name + ': missing file ' + href);
    }
  });

  Array.prototype.forEach.call(d.forms, function (f) {
    if (/\bw\b/.test(f.className)) {
      widgets++;
      var out = d.getElementById(f.id + '-out');
      assert(out && out.innerHTML.length > 20, name + ': widget ' + f.id + ' produced no output');
      assert(!/Could not calculate|Check the inputs/.test(out.innerHTML), name + ': widget ' + f.id + ' failed: ' + out.innerHTML);
      if (f.elements.kind.value === 'extraction') {
        assert(f.querySelector('td.hit'), name + ': extraction chart not marked for ' + f.id);
        assert(f.querySelectorAll('td.line').length > 3, name + ': ratio line not drawn for ' + f.id);
      }
    }
    if (/quiz-form/.test(f.className)) {
      quizzes++;
      var key = f.elements['quiz-key'].value, n = Number(f.elements['quiz-n'].value), i;
      for (i = 0; i < n; i++) {
        var qn = key + '-' + i, ans = f.elements[qn + '-ans'].value;
        if (f.elements[qn + '-kind'].value === 'choice') {
          d.getElementById(f.id.replace(key, '') + qn + '-' + ans).checked = true;
        } else {
          f.elements[qn].value = ans;
        }
      }
      var score = w.UB.checkQuiz(f);
      assert.strictEqual(score, n, name + ': quiz ' + f.id + ' scored ' + score + '/' + n);
      for (i = 0; i < n; i++) {
        var fb = d.getElementById(f.id + '-' + i + '-fb');
        assert(/right/.test(fb.className) && fb.innerHTML.length > 12, name + ': feedback missing for ' + f.id + '-' + i);
      }
      questions += n;
      // A wrong numeric answer must be marked wrong.
      for (i = 0; i < n; i++) {
        qn = key + '-' + i;
        if (f.elements[qn + '-kind'].value === 'numeric') {
          f.elements[qn].value = '99999';
          w.UB.checkQuiz(f);
          assert(/wrong/.test(d.getElementById(f.id + '-' + i + '-fb').className), name + ': wrong answer accepted');
          break;
        }
      }
    }
  });
});

console.log('pages: ' + pages.length + ', widgets: ' + widgets + ', quizzes: ' + quizzes + ', questions: ' + questions + '; all checks passed');
