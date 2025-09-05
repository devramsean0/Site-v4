import { Application } from "@hotwired/stimulus";
import definitions from 'stimulus:./controllers';

const application = Application.start();
application.load(definitions)