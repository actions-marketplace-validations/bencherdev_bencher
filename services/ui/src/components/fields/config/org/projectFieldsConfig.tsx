import validateDescription from "../../validators/validateDescription";
import validateName from "../../validators/validateName";
import validateSlug from "../../validators/validateSlug";
import validator from "validator";

const projectFieldsConfig = {
  name: {
    label: "Name",
    type: "text",
    placeholder: "Project Name",
    icon: "fas fa-project-diagram",
    help: "Must be at least four characters or longer.",
    validate: validateName,
  },
  slug: {
    label: "Slug",
    type: "text",
    placeholder: "Project Slug",
    icon: "fas fa-exclamation-triangle",
    help: "Must be at least four characters or longer.",
    validate: validateSlug,
  },
  description: {
    label: "Description",
    type: "textarea",
    placeholder: "Describe the project",
    help: "Must be between 25 and 2,500 characters.",
    validate: validateDescription,
  },
  url: {
    label: "URL",
    type: "text",
    placeholder: "www.example.com",
    icon: "fas fa-link",
    help: "Must be a valid public facing URL.",
    validate: validator.isURL,
  },
  public: {
    label: "Public Project",
    type: "checkbox",
  },
};

export default projectFieldsConfig;